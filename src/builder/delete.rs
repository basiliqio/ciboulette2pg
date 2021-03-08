use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    fn gen_delete_normal(
        &mut self,
        table_store: &'a Ciboulette2PostgresTableStore<'a>,
        query: &'a CibouletteDeleteRequest<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        let main_table = table_store.get(query.resource_type().name().as_str())?;

        self.buf.write_all(b"DELETE FROM ")?;
        self.write_table_info(main_table)?;
        self.buf.write_all(b" WHERE ")?;
        self.insert_ident(&(main_table.id_name().clone(), None, None), &main_table)?;
        self.buf.write_all(b" = ")?;
        self.insert_params(
            Ciboulette2SqlValue::Text(Some(Cow::Borrowed(query.resource_id().as_ref()))),
            &main_table,
        )?;
        Ok(())
    }

    fn gen_delete_rel(
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

    pub fn gen_delete(
        &mut self,
        store: &'a CibouletteStore<'a>,
        table_store: &'a Ciboulette2PostgresTableStore<'a>,
        query: &'a CibouletteDeleteRequest<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        match query.related_type() {
            Some(related_type) => {
                let alias = query
                    .resource_type()
                    .get_alias(related_type.name().as_str())?;
                let (_, opt) =
                    store.get_rel(query.resource_type().name().as_str(), alias.as_str())?;
                match opt {
                    CibouletteRelationshipOption::One(opt) if *opt.optional() => {
                        self.gen_delete_rel(&table_store, query, opt)
                    }
                    CibouletteRelationshipOption::One(opt) => {
                        Err(Ciboulette2SqlError::RequiredRelationship(
                            query.resource_type().name().clone(),
                            opt.key().clone(),
                        ))
                    }
                    _ => Err(Ciboulette2SqlError::BulkRelationshipDelete),
                }
            }
            None => self.gen_delete_normal(&table_store, query),
        }
    }
}
