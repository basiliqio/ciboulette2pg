use super::*;

impl<'request> Ciboulette2PgBuilder<'request> {
    /// Delete a many-to-one relationships
    ///
    /// Updating the "many" table, to replace the foreign id by a `NULL`
    pub(super) fn gen_delete_rel_one_to_many(
        &mut self,
        table_store: &Ciboulette2PgTableStore,
        query: &'request CibouletteDeleteRequest<'request>,
        rel_opt: &CibouletteRelationshipOneToManyOption,
    ) -> Result<(), Ciboulette2PgError> {
        let many_table = table_store.get(rel_opt.many_resource().name().as_str())?;

        self.buf.write_all(b"UPDATE ")?;
        self.write_table_info(many_table)?;
        self.buf.write_all(b" SET ")?;
        self.insert_ident_name(
            &Ciboulette2PgTableField::new(
                Ciboulette2PgSafeIdent::try_from(rel_opt.many_resource_key())?,
                None,
                None,
            ),
            &many_table,
        )?;
        self.buf.write_all(b" = NULL WHERE ")?;
        self.insert_ident(
            &Ciboulette2PgTableField::new(many_table.id().get_ident().clone(), None, None),
            &many_table,
        )?;
        self.buf.write_all(b" = ")?;
        self.insert_params(Ciboulette2PgValue::from(query.resource_id()), &many_table)?;
        Ok(())
    }
}
