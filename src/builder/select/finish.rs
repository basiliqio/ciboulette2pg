use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(crate) fn finish_request(
        &mut self,
        state: Ciboulette2PostgresBuilderState<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        let (main_cte_table, _) = self.working_tables.get(state.main_table()).ok_or_else(|| {
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
                &Ciboulette2PostgresTableField::new_ref(&CIBOULETTE_ID_IDENT, None, None),
                table,
                None,
            )?;
            self.buf.write_all(b", ")?;
            Self::insert_ident_inner(
                &mut self.buf,
                &Ciboulette2PostgresTableField::new_ref(&CIBOULETTE_TYPE_IDENT, None, None),
                table,
                None,
            )?;
            self.buf.write_all(b", ")?;
            Self::insert_ident_inner(
                &mut self.buf,
                &Ciboulette2PostgresTableField::new_ref(&CIBOULETTE_DATA_IDENT, None, None),
                table,
                None,
            )?;
            self.buf.write_all(b", ")?;
            Self::insert_ident_inner(
                &mut self.buf,
                &Ciboulette2PostgresTableField::new_ref(&CIBOULETTE_RELATED_TYPE_IDENT, None, None),
                table,
                None,
            )?;
            self.buf.write_all(b", ")?;
            Self::insert_ident_inner(
                &mut self.buf,
                &Ciboulette2PostgresTableField::new_ref(&CIBOULETTE_RELATED_ID_IDENT, None, None),
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
    pub(crate) fn gen_select_cte_final<'b, I>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        table: &Ciboulette2PostgresTable<'a>,
        type_: Arc<CibouletteResourceType<'a>>,
        relating_field: Option<Ciboulette2PostgresRelatingField<'a>>,
        additional_fields: I,
        include: bool,
    ) -> Result<(), Ciboulette2SqlError>
    where
        'a: 'b,
        I: Iterator<Item = &'b Ciboulette2SqlAdditionalField<'a>>,
    {
        self.buf.write_all(b"SELECT ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_cow(
                Cow::Borrowed(table.id().get_ident()),
                Some(Cow::Owned(Ciboulette2PostgresSafeIdent::try_from("id")?)),
                Some(Cow::Owned(Ciboulette2PostgresSafeIdent::try_from("TEXT")?)),
            ),
            table,
        )?;
        self.buf.write_all(b", ")?;
        self.insert_params(
            Ciboulette2SqlValue::Text(Some(ArcCowStr::Arc(type_.name().clone()))),
            table,
        )?;
        self.buf.write_all(b"::TEXT AS \"type\", ")?;
        self.gen_json_builder(table, type_.clone(), state.query(), include)?;
        self.buf.write_all(b" AS \"data\", ")?;
        match relating_field {
            Some(relating_field) => {
                self.insert_ident(relating_field.field(), relating_field.table())?;
                self.buf.write_all(b"::TEXT AS \"related_id\", ")?;
                self.insert_params(
                    Ciboulette2SqlValue::Text(Some(ArcCowStr::Arc(
                        relating_field.related_type().name().clone(),
                    ))), // TODO do better
                    relating_field.table(),
                )?;
                self.buf.write_all(b"::TEXT AS \"related_type\"")?;
            }
            None => {
                self.buf
                    .write_all(b"NULL::TEXT AS \"related_id\", NULL::TEXT AS \"related_type\"")?;
            }
        }
        self.handle_additionnal_params(&state, &table, additional_fields)?;
        self.gen_sorting_keys(&table, type_.clone(), &state.query())?;
        self.buf.write_all(b" FROM ")?;
        self.write_table_info(table)?;
        Ok(())
    }
}
