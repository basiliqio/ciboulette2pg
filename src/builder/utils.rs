use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(crate) fn gen_rel_values(
        &mut self,
        ids: Vec<value::Ciboulette2SqlValue<'a>>,
        table: &'a Ciboulette2PostgresTableSettings<'a>,
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

    #[inline]
    pub fn build(mut self) -> Result<(String, Ciboulette2SqlArguments<'a>), Ciboulette2SqlError> {
        self.buf.write_all(b";")?;
        Ok((
            String::from_utf8(self.buf.into_inner()?.into_inner())?,
            self.params,
        ))
    }

    #[inline]
    pub(crate) fn insert_ident_inner(
        buf: &mut Ciboulette2PostgresBuf,
        field: &Ciboulette2PostgresTableField,
        table: &Ciboulette2PostgresTableSettings,
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

    #[inline]
    pub(crate) fn insert_ident(
        &mut self,
        field: &Ciboulette2PostgresTableField,
        table: &Ciboulette2PostgresTableSettings,
    ) -> Result<(), Ciboulette2SqlError> {
        Self::insert_ident_inner(&mut self.buf, &field, table, None)
    }

    #[inline]
    pub(crate) fn insert_ident_name(
        &mut self,
        field: &Ciboulette2PostgresTableField,
        _table: &Ciboulette2PostgresTableSettings,
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

    #[inline]
    pub(crate) fn insert_params(
        &mut self,
        param: Ciboulette2SqlValue<'a>,
        _table: &Ciboulette2PostgresTableSettings<'_>,
    ) -> Result<(), Ciboulette2SqlError> {
        let mut buffer = [0u8; 20];
        self.params.push(param);
        let len = self.params.len();

        self.buf.write_all(b"$")?;
        self.buf.write_all(len.numtoa(10, &mut buffer))?;
        Ok(())
    }

    #[inline]
    pub(crate) fn handle_sorting_routine(
        mut buf: &mut Ciboulette2PostgresBuf,
        state: &Ciboulette2PostgresBuilderState<'a>,
        main_cte_table: &Ciboulette2PostgresTableSettings<'a>,
        table: &Ciboulette2PostgresTableSettings<'a>,
        included_tables_map: &BTreeMap<
            &'a Ciboulette2PostgresTableSettings<'a>,
            Ciboulette2PostgresTableSettings<'a>,
        >,
    ) -> Result<(), Ciboulette2SqlError> {
        if main_cte_table != table {
            return Ok(());
        }
        let mut included_tables: Vec<&Ciboulette2PostgresTableSettings<'a>> =
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
            let included_table = included_tables_map
                .get(state.table_store().get(el.type_().name().as_str())?)
                .ok_or_else(|| {
                    Ciboulette2SqlError::MissingRelationForOrdering(table.name().to_string())
                })?;
            Self::gen_sort_inner_joins(
                buf,
                &state.table_store(),
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
                &Ciboulette2PostgresTableField::new_owned(
                    Ciboulette2PostgresSafeIdent::try_from(
                        format!("sort_{}", el.field().as_ref()).as_str(),
                    )?,
                    None,
                    None,
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

    #[inline]
    pub(crate) fn write_table_info_inner(
        buf: &mut Ciboulette2PostgresBuf,
        table: &Ciboulette2PostgresTableSettings,
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

    #[inline]
    pub(crate) fn write_table_info(
        &mut self,
        table: &Ciboulette2PostgresTableSettings,
    ) -> Result<(), Ciboulette2SqlError> {
        Self::write_table_info_inner(&mut self.buf, table)
    }

    pub(crate) fn write_list<I, F>(
        &mut self,
        arr: I,
        table: &Ciboulette2PostgresTableSettings,
        wrap_in_parenthesis: bool,
        f: F,
    ) -> Result<(), Ciboulette2SqlError>
    where
        I: std::iter::IntoIterator,
        F: for<'r> Fn(
            &'r mut Ciboulette2PostgresBuilder<'a>,
            I::Item,
            &Ciboulette2PostgresTableSettings,
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

    pub(crate) fn compare_fields(
        &mut self,
        left_table: &Ciboulette2PostgresTableSettings,
        left: &Ciboulette2PostgresTableField,
        right_table: &Ciboulette2PostgresTableSettings,
        right: &Ciboulette2PostgresTableField,
    ) -> Result<(), Ciboulette2SqlError> {
        Self::insert_ident_inner(&mut self.buf, left, &left_table, Some("TEXT"))?;
        self.buf.write_all(b" = ")?;
        Self::insert_ident_inner(&mut self.buf, &right, &right_table, Some("TEXT"))?;
        //FIXME Make a better id type management system
        Ok(())
    }

    // pub(crate) fn should_include_type(
    //     ciboulette_store: &'a CibouletteStore<'a>,
    //     query: &CibouletteQueryParameters<'a>,
    //     path: &CiboulettePath<'a>,
    //     type_: &CibouletteResourceType<'a>,
    // ) -> bool {
    //     query.include().contains(type_)
    //         || query.sorting_map().contains_key(type_)
    //         || match path {
    //             CiboulettePath::Type(x) => x == &type_,
    //             CiboulettePath::TypeId(x, _) => x == &type_,
    //             CiboulettePath::TypeIdRelated(x, y, z) => x == &type_ || z == &type_,
    //             CiboulettePath::TypeIdRelationship(x, y, z) => {
    //                 x == &type_
    //                     || ciboulette_store
    //                         .get_rel(x.name().as_str(), y.name().as_str())
    //                         .and_then(|(_, opt)| match opt {
    //                             CibouletteRelationshipOption::One(opt) => z == &type_,
    //                             CibouletteRelationshipOption::Many(opt) => z == &type_,
    // 							_ => false
    //                         })
    //             }
    //         }
    // }
}
