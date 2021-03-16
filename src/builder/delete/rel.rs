use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(super) fn gen_delete_rel(
        &mut self,
        table_store: &'a Ciboulette2PostgresTableStore<'a>,
        query: &'a CibouletteDeleteRequest<'a>,
        rel_opt: &CibouletteRelationshipOneToOneOption,
    ) -> Result<(), Ciboulette2SqlError> {
        let main_table = table_store.get(query.resource_type().name().as_str())?;

        self.buf.write_all(b"UPDATE ")?;
        self.write_table_info(main_table)?;
        self.buf.write_all(b" SET ")?;
        self.insert_ident_name(
            &Ciboulette2PostgresTableField::new_owned(
                Ciboulette2PostgresSafeIdent::try_from(rel_opt.key().as_str())?,
                None,
                None,
            ),
            &main_table,
        )?;
        self.buf.write_all(b" = NULL WHERE ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(main_table.id().get_ident(), None, None),
            &main_table,
        )?;
        self.buf.write_all(b" = ")?;
        self.insert_params(Ciboulette2SqlValue::from(query.resource_id()), &main_table)?;
        Ok(())
    }
}
