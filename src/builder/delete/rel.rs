use super::*;

impl<'store, 'request> Ciboulette2PostgresBuilder<'store, 'request>
where
    'store: 'request,
{
    pub(super) fn gen_delete_rel_one_to_many(
        &mut self,
        table_store: &'store Ciboulette2PostgresTableStore<'store>,
        query: &'request CibouletteDeleteRequest<'request, 'store>,
        rel_opt: &CibouletteRelationshipOneToManyOption,
    ) -> Result<(), Ciboulette2SqlError> {
        let many_table = table_store.get(rel_opt.many_table().name().as_str())?;

        self.buf.write_all(b"UPDATE ")?;
        self.write_table_info(many_table)?;
        self.buf.write_all(b" SET ")?;
        self.insert_ident_name(
            &Ciboulette2PostgresTableField::new_owned(
                Ciboulette2PostgresSafeIdent::try_from(rel_opt.many_table_key().as_str())?,
                None,
                None,
            ),
            &many_table,
        )?;
        self.buf.write_all(b" = NULL WHERE ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(many_table.id().get_ident(), None, None),
            &many_table,
        )?;
        self.buf.write_all(b" = ")?;
        self.insert_params(Ciboulette2SqlValue::from(query.resource_id()), &many_table)?;
        Ok(())
    }
}
