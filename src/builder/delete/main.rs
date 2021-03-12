use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(super) fn gen_delete_normal(
        &mut self,
        table_store: &'a Ciboulette2PostgresTableStore<'a>,
        query: &'a CibouletteDeleteRequest<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        let main_table = table_store.get(query.resource_type().name().as_str())?;

        self.buf.write_all(b"DELETE FROM ")?;
        self.write_table_info(main_table)?;
        self.buf.write_all(b" WHERE ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(main_table.id_name(), None, None),
            &main_table,
        )?;
        self.buf.write_all(b" = ")?;
        self.insert_params(
            Ciboulette2SqlValue::Text(Some(Cow::Borrowed(query.resource_id().as_ref()))),
            &main_table,
        )?;
        Ok(())
    }
}
