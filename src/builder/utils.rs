use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(crate) fn gen_rel_values(
        &mut self,
        ids: Vec<value::Ciboulette2SqlValue<'a>>,
        table: &'a Ciboulette2PostgresTableSettings<'a>,
        type_: &str,
    ) -> Result<(), Ciboulette2SqlError> {
        // It's a logic error to have an empty id vector here
        if ids.is_empty() {
            return Err(Ciboulette2SqlError::EmptyRelValue(type_.to_string()));
        }
        // ($x::type), ($x::type), ($x::type)
        self.write_list(ids, &table, false, |ref mut se, curr, t| {
            se.buf.write_all(b"(")?;
            se.insert_params(curr, t)?;
            se.buf.write_all(b"::")?;
            se.buf.write_all(type_.as_bytes())?;
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
        (ident, alias, cast): &(
            &Ciboulette2PostgresSafeIdent,
            &Option<Ciboulette2PostgresSafeIdent>,
            &Option<Ciboulette2PostgresSafeIdent>,
        ),
        table: &Ciboulette2PostgresTableSettings,
    ) -> Result<(), Ciboulette2SqlError> {
        Self::write_table_info_inner(buf, table)?;
        buf.write_all(b".")?;
        buf.write_all(POSTGRES_QUOTE)?;
        buf.write_all(ident.as_bytes())?;
        buf.write_all(POSTGRES_QUOTE)?;
        match cast {
            Some(cast) => {
                buf.write_all(b"::")?;
                buf.write_all(cast.as_bytes())?;
            }
            None => (),
        };
        match alias {
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
        (ident, alias, cast): &(
            Ciboulette2PostgresSafeIdent,
            Option<Ciboulette2PostgresSafeIdent>,
            Option<Ciboulette2PostgresSafeIdent>,
        ),
        table: &Ciboulette2PostgresTableSettings,
    ) -> Result<(), Ciboulette2SqlError> {
        Self::insert_ident_inner(&mut self.buf, &(ident, alias, cast), table)
    }

    #[inline]
    pub(crate) fn insert_ident_name(
        &mut self,
        (ident, alias, cast): &(
            Ciboulette2PostgresSafeIdent,
            Option<Ciboulette2PostgresSafeIdent>,
            Option<Ciboulette2PostgresSafeIdent>,
        ),
        _table: &Ciboulette2PostgresTableSettings,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write_all(POSTGRES_QUOTE)?;
        self.buf.write_all(ident.as_bytes())?;
        self.buf.write_all(POSTGRES_QUOTE)?;
        match cast {
            Some(cast) => {
                self.buf.write_all(b"::")?;
                self.buf.write_all(cast.as_bytes())?;
            }
            None => (),
        };
        match alias {
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
        let old_len = self.params.len();

        self.params.push(param);
        self.buf.write_all(b"$")?;
        self.buf.write_all(old_len.numtoa(10, &mut buffer))?;
        Ok(())
    }

    #[inline]
    pub(crate) fn handle_sorting_routine(
        mut buf: &mut Ciboulette2PostgresBuf,
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        query: &CibouletteQueryParameters<'a>,
        main_table: &Ciboulette2PostgresTableSettings<'a>,
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
            Vec::with_capacity(query.sorting_map().len());

        for el in query.sorting() {
            if el.type_() == main_table.ciboulette_type() {
                included_tables.push(main_table);
                continue;
            }
            let (_, opt) = ciboulette_store.get_rel(
                main_table.ciboulette_type().name().as_str(),
                el.type_().name().as_str(),
            )?;
            let included_table = included_tables_map
                .get(ciboulette_table_store.get(el.type_().name().as_str())?)
                .ok_or_else(|| {
                    Ciboulette2SqlError::MissingRelationForOrdering(table.name().to_string())
                })?;
            Self::gen_sort_inner_joins(
                buf,
                &ciboulette_table_store,
                &included_table,
                &main_table,
                &main_cte_table,
                &opt,
            )?;
            included_tables.push(included_table);
        }

        let mut iter = query.sorting().iter().zip(included_tables).peekable();
        if iter.peek().is_some() {
            buf.write_all(b" ORDER BY ")?;
        }
        while let Some((el, table)) = iter.next() {
            Self::insert_ident_inner(
                &mut buf,
                &(
                    &Ciboulette2PostgresSafeIdent::try_from(
                        format!("sort_{}", el.field().as_ref()).as_str(),
                    )?,
                    &None,
                    &None,
                ),
                table,
            )?;
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
}
