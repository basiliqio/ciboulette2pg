use super::*;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    pub(crate) fn finish_request<'store>(
        &mut self,
        state: Ciboulette2PostgresBuilderState<'store, 'request>,
    ) -> Result<(), Ciboulette2SqlError> {
        let (main_cte_table, _) = self
            .working_tables
            .get(state.main_table().name())
            .ok_or_else(|| {
                CibouletteError::UnknownError("Can't find the main_cte_table".to_string())
            })?;
        let mut first_one = true;
        for (table, is_needed) in self.working_tables.values() {
            if matches!(is_needed, Ciboulette2PostgresResponseType::None) {
                continue;
            } else if !first_one {
                self.buf.write_all(b" UNION ALL ")?;
            } else {
                first_one = false;
            }
            // SELECT * FROM
            self.buf.write_all(b"(SELECT ")?;
            Self::insert_ident_inner(
                &mut self.buf,
                &Ciboulette2PostgresTableField::new(CIBOULETTE_ID_IDENT, None, None),
                table,
                None,
            )?;
            self.buf.write_all(b", ")?;
            Self::insert_ident_inner(
                &mut self.buf,
                &Ciboulette2PostgresTableField::new(CIBOULETTE_TYPE_IDENT, None, None),
                table,
                None,
            )?;
            self.buf.write_all(b", ")?;
            Self::insert_ident_inner(
                &mut self.buf,
                &Ciboulette2PostgresTableField::new(CIBOULETTE_DATA_IDENT, None, None),
                table,
                None,
            )?;
            self.buf.write_all(b", ")?;
            Self::insert_ident_inner(
                &mut self.buf,
                &Ciboulette2PostgresTableField::new(CIBOULETTE_RELATED_TYPE_IDENT, None, None),
                table,
                None,
            )?;
            self.buf.write_all(b", ")?;
            Self::insert_ident_inner(
                &mut self.buf,
                &Ciboulette2PostgresTableField::new(CIBOULETTE_RELATED_ID_IDENT, None, None),
                table,
                None,
            )?;
            self.buf.write_all(b" FROM ")?;
            // SELECT * FROM "schema"."mytable"
            Self::write_table_info_inner(&mut self.buf, table)?;
            Self::handle_sorting_routine(
                &mut self.buf,
                &state,
                &main_cte_table,
                table,
                &self.working_tables,
            )?;
            self.buf.write_all(b")")?;
        }
        Ok(())
    }
    pub(crate) fn gen_select_cte_final<'store, 'b, I>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        table: &Ciboulette2PostgresTable,
        type_: Arc<CibouletteResourceType>,
        relating_field: Option<Ciboulette2PostgresRelatingField>,
        additional_fields: I,
        include: bool,
    ) -> Result<(), Ciboulette2SqlError>
    where
        I: Iterator<Item = &'b Ciboulette2SqlAdditionalField>,
    {
        self.buf.write_all(b"SELECT ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new(
                table.id().get_ident().clone(),
                Some(CIBOULETTE_ID_IDENT),
                Some(TEXT_IDENT),
            ),
            table,
        )?;
        self.buf.write_all(b", ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new(
                table.id().get_ident().clone(),
                Some(CIBOULETTE_MAIN_IDENTIFIER),
                None,
            ),
            table,
        )?;
        self.buf.write_all(b", ")?;
        match relating_field {
            Some(relating_field) => {
                self.insert_ident(relating_field.field(), relating_field.table())?;
                self.buf.write_all(b"::TEXT AS \"related_id\", ")?;
                self.insert_params(
                    Ciboulette2SqlValue::ArcStr(Some(relating_field.related_type().name().clone())),
                    relating_field.table(),
                )?;
                self.buf.write_all(b"::TEXT AS \"related_type\", ")?;
                self.insert_params(
                    Ciboulette2SqlValue::ArcStr(Some(relating_field.alias().clone())),
                    table,
                )?;
            }
            None => {
                self.buf
                    .write_all(b"NULL::TEXT AS \"related_id\", NULL::TEXT AS \"related_type\", ")?;
                self.insert_params(
                    Ciboulette2SqlValue::ArcStr(Some(type_.name().clone())),
                    table,
                )?;
            }
        }
        self.buf.write_all(b"::TEXT AS \"type\", ")?;
        self.gen_json_builder(table, type_.clone(), state.query(), include)?;
        self.buf.write_all(b" AS \"data\"")?;
        self.handle_additionnal_params(&table, additional_fields)?;
        self.gen_sorting_keys(&table, type_, &state.query())?;
        self.buf.write_all(b" FROM ")?;
        self.write_table_info(table)?;
        Ok(())
    }
}
