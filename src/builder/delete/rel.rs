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
        self.insert_ident(
            &(
                Ciboulette2PostgresSafeIdent::try_from(rel_opt.key().as_str())?,
                None,
                None,
            ),
            &main_table,
        )?;
        self.buf.write_all(b" = NULL WHERE ")?;
        self.insert_ident(&(main_table.id_name().clone(), None, None), &main_table)?;
        self.insert_params(
            Ciboulette2SqlValue::Text(Some(Cow::Borrowed(query.resource_id().as_ref()))),
            &main_table,
        )?;
        Ok(())
    }
}