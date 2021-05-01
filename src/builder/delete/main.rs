use super::*;

impl<'store, 'request> Ciboulette2PgBuilder<'request>
where
    'store: 'request,
{
    /// Generate a SQL query to delete a single object from the database
    ///
    /// Generated when receiving a request like `DELETE /peoples/{id}`
    pub(super) fn gen_delete_normal(
        &mut self,
        table_store: &Ciboulette2PgTableStore,
        query: &'request CibouletteDeleteRequest<'request>,
    ) -> Result<(), Ciboulette2PgError> {
        let main_table = table_store.get(query.resource_type().name().as_str())?;
        self.buf.write_all(b"DELETE FROM ")?;
        self.write_table_info(main_table)?;
        self.buf.write_all(b" WHERE ")?;
        self.insert_ident(
            &Ciboulette2PgTableField::new(main_table.id().get_ident().clone(), None, None),
            &main_table,
        )?;
        self.buf.write_all(b" = ")?;
        self.insert_params(Ciboulette2PgValue::from(query.resource_id()), &main_table)?;
        Ok(())
    }
}
