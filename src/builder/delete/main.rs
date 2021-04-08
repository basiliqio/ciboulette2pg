use super::*;

impl<'store, 'request> Ciboulette2PostgresBuilder<'store, 'request>
where
    'store: 'request,
{
    /// Generate a SQL query to delete a single object from the database
    ///
    /// Generated when receiving a request like `DELETE /peoples/{id}`
    pub(super) fn gen_delete_normal(
        &mut self,
        table_store: &'store Ciboulette2PostgresTableStore<'store>,
        query: &'request CibouletteDeleteRequest<'request, 'store>,
    ) -> Result<(), Ciboulette2SqlError> {
        let main_table = table_store.get(query.resource_type().name().as_str())?;

        self.buf.write_all(b"DELETE FROM ")?;
        self.write_table_info(main_table)?;
        self.buf.write_all(b" WHERE ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(main_table.id().get_ident(), None, None),
            &main_table,
        )?;
        self.buf.write_all(b" = ")?;
        self.insert_params(Ciboulette2SqlValue::from(query.resource_id()), &main_table)?;
        Ok(())
    }
}
