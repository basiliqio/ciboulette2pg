use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    /// Create a list of relationships' id, to use as value for later inserting
    pub(crate) fn gen_rel_values(
        &mut self,
        ids: Vec<value::Ciboulette2SqlValue<'a>>,
        table: &'a Ciboulette2PostgresTable<'a>,
        id_param: &Ciboulette2PostgresId,
    ) -> Result<(), Ciboulette2SqlError> {
        // It's a logic error to have an empty id vector here
        if ids.is_empty() {
            return Err(Ciboulette2SqlError::EmptyRelValue(
                id_param.get_ident().to_string(),
            ));
        }
        self.write_list(ids, &table, false, |ref mut se, curr, t| {
            se.buf.write_all(b"(")?;
            se.insert_params(curr, t)?;
            se.buf.write_all(b"::")?;
            se.buf.write_all(id_param.get_type().as_bytes())?;
            se.buf.write_all(b")")?;
            Ok(())
        })?;
        Ok(())
    }

    /// Finish building the query, setting the last ';' and then converting the
    /// final query to UTF-8
    #[inline]
    pub fn build(mut self) -> Result<(String, Ciboulette2SqlArguments<'a>), Ciboulette2SqlError> {
        self.buf.write_all(b";")?;
        Ok((
            String::from_utf8(self.buf.into_inner()?.into_inner())?, // TODO Is it the best way to handle that ?
            self.params,
        ))
    }

    /// Inserts an identifier to the query.
    ///
    /// In the form `"schema"."table"."ident"[::CAST] [AS "ALIAS"]`
    #[inline]
    pub(crate) fn insert_ident_inner(
        buf: &mut Ciboulette2PostgresBuf,
        field: &Ciboulette2PostgresTableField,
        table: &Ciboulette2PostgresTable,
        force_cast: Option<&'static str>,
    ) -> Result<(), Ciboulette2SqlError> {
        Self::write_table_info_inner(buf, table)?;
        buf.write_all(b".")?;
        buf.write_all(POSTGRES_QUOTE)?;
        buf.write_all(field.name().as_bytes())?;
        buf.write_all(POSTGRES_QUOTE)?;
        if let Some(force_cast) = force_cast {
            buf.write_all(b"::")?;
            buf.write_all(force_cast.as_bytes())?;
        } else {
            match field.cast() {
                Some(cast) => {
                    buf.write_all(b"::")?;
                    buf.write_all(cast.as_bytes())?;
                }
                None => (),
            };
        }
        match field.alias() {
            Some(alias) => {
                buf.write_all(b" AS ")?;
                buf.write_all(POSTGRES_QUOTE)?;
                buf.write_all(alias.as_bytes())?;
                buf.write_all(POSTGRES_QUOTE)?;
            }
            None => (),
        };
        Ok(())
    }

    /// Wrapper for [insert_ident_inner](Self::insert_ident_inner)
    #[inline]
    pub(crate) fn insert_ident(
        &mut self,
        field: &Ciboulette2PostgresTableField,
        table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        Self::insert_ident_inner(&mut self.buf, &field, table, None)
    }

    /// Inserts an identifier name to the query.
    ///
    /// In the form `"ident"`[::CAST] [AS "ALIAS"]
    #[inline]
    pub(crate) fn insert_ident_name(
        &mut self,
        field: &Ciboulette2PostgresTableField,
        _table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write_all(POSTGRES_QUOTE)?;
        self.buf.write_all(field.name().as_bytes())?;
        self.buf.write_all(POSTGRES_QUOTE)?;
        match field.cast() {
            Some(cast) => {
                self.buf.write_all(b"::")?;
                self.buf.write_all(cast.as_bytes())?;
            }
            None => (),
        };
        match field.alias() {
            Some(alias) => {
                self.buf.write_all(b" AS ")?;
                self.buf.write_all(POSTGRES_QUOTE)?;
                self.buf.write_all(alias.as_bytes())?;
                self.buf.write_all(POSTGRES_QUOTE)?;
            }
            None => (),
        };
        Ok(())
    }

    /// Inserts a parameter into the query in the form of `$n` with n
    /// incremented from 1
    #[inline]
    pub(crate) fn insert_params(
        &mut self,
        param: Ciboulette2SqlValue<'a>,
        _table: &Ciboulette2PostgresTable<'_>,
    ) -> Result<(), Ciboulette2SqlError> {
        let mut buffer = [0u8; 20];
        self.params.push(param);
        let len = self.params.len();

        self.buf.write_all(b"$")?;
        self.buf.write_all(len.numtoa(10, &mut buffer))?;
        Ok(())
    }

    /// Handle joigning the related tables + ordering the result set depending
    /// on the sorting requirement set by the request
    #[inline]
    pub(crate) fn handle_sorting_routine(
        mut buf: &mut Ciboulette2PostgresBuf,
        state: &Ciboulette2PostgresBuilderState<'a>,
        main_cte_table: &Ciboulette2PostgresTable<'a>,
        table: &Ciboulette2PostgresTable<'a>,
        included_tables_map: &BTreeMap<
            &'a Ciboulette2PostgresTable<'a>,
            (Ciboulette2PostgresTable<'a>, CibouletteResponseRequiredType),
        >,
    ) -> Result<(), Ciboulette2SqlError> {
        if main_cte_table != table {
            return Ok(());
        }
        let mut included_tables: Vec<&Ciboulette2PostgresTable<'a>> =
            Vec::with_capacity(state.query().sorting_map().len());

        for el in state.query().sorting() {
            if el.type_() == state.main_type() {
                included_tables.push(main_cte_table);
                continue;
            }
            let (_, opt) = state.store().get_rel(
                state.main_type().name().as_str(),
                el.type_().name().as_str(),
            )?;
            let (included_table, _) = included_tables_map
                .get(state.table_store().get(el.type_().name().as_str())?)
                .ok_or_else(|| {
                    Ciboulette2SqlError::MissingRelationForOrdering(table.name().to_string())
                })?;
            Self::gen_sort_joins(
                buf,
                &included_table,
                &state.main_table(),
                &main_cte_table,
                &opt,
            )?;
            included_tables.push(included_table);
        }

        let mut iter = state
            .query()
            .sorting()
            .iter()
            .zip(included_tables)
            .peekable();
        if iter.peek().is_some() {
            buf.write_all(b" ORDER BY ")?;
        }
        while let Some((el, table)) = iter.next() {
            Self::insert_ident_inner(
                &mut buf,
                &Ciboulette2PostgresTableField::from_additional_field_with_cast(
                    Ciboulette2SqlAdditionalField::try_from(el)?,
                ),
                table,
                None,
            )?;
            match el.direction() {
                CibouletteSortingDirection::Asc => buf.write_all(b" ASC")?,
                CibouletteSortingDirection::Desc => buf.write_all(b" DESC")?,
            }
            if iter.peek().is_some() {
                buf.write_all(b", ")?;
            }
        }
        Ok(())
    }

    /// Write the table information to the query
    ///
    /// In the form of `"schema"."table"`
    #[inline]
    pub(crate) fn write_table_info_inner(
        buf: &mut Ciboulette2PostgresBuf,
        table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        buf.write_all(POSTGRES_QUOTE)?;
        match table.schema() {
            Some(x) => {
                buf.write_all(x.as_bytes())?;
                buf.write_all(b"\".\"")?;
            }
            None => (),
        };
        buf.write_all(table.name().as_bytes())?;
        buf.write_all(POSTGRES_QUOTE)?;
        Ok(())
    }

    /// Wrapper for [write_table_info_inner](Self::write_table_info_inner)
    #[inline]
    pub(crate) fn write_table_info(
        &mut self,
        table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        Self::write_table_info_inner(&mut self.buf, table)
    }

    /// Write a list of something to the query.
    /// The something part will be determined by a callback function.
    /// The result can be wrapped in parenthesis and will be comma-separated
    pub(crate) fn write_list<I, F>(
        &mut self,
        arr: I,
        table: &Ciboulette2PostgresTable,
        wrap_in_parenthesis: bool,
        f: F,
    ) -> Result<(), Ciboulette2SqlError>
    where
        I: std::iter::IntoIterator,
        F: for<'r> Fn(
            &'r mut Ciboulette2PostgresBuilder<'a>,
            I::Item,
            &Ciboulette2PostgresTable,
        ) -> Result<(), Ciboulette2SqlError>,
    {
        let mut iter = arr.into_iter().peekable();
        if wrap_in_parenthesis {
            self.buf.write_all(b"(")?;
        }
        while let Some(curr) = iter.next() {
            f(self, curr, &table)?;

            if iter.peek().is_some() {
                self.buf.write_all(b", ")?;
            }
        }
        if wrap_in_parenthesis {
            self.buf.write_all(b")")?;
        }
        Ok(())
    }

    /// Compare 2 fields, casting them to `TEXT` to be sure of type compatibility
    pub(crate) fn compare_fields(
        &mut self,
        left_table: &Ciboulette2PostgresTable,
        left: &Ciboulette2PostgresTableField,
        right_table: &Ciboulette2PostgresTable,
        right: &Ciboulette2PostgresTableField,
    ) -> Result<(), Ciboulette2SqlError> {
        Self::insert_ident_inner(&mut self.buf, left, &left_table, Some("TEXT"))?;
        self.buf.write_all(b" = ")?;
        Self::insert_ident_inner(&mut self.buf, &right, &right_table, Some("TEXT"))?;
        //FIXME Make a better id type management system
        Ok(())
    }
}
