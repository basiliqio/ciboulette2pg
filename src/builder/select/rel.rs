use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(crate) fn gen_select_single_rel_routine(
        &mut self,
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        query: &'a CibouletteQueryParameters<'a>,
        main_type: &'a CibouletteResourceType<'a>,
        main_cte_data: &Ciboulette2PostgresTableSettings<'a>,
        rels: &Ciboulette2SqlQueryRels<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        for (rel_key, additional_fields) in rels
            .single_rels_keys()
            .iter()
            .zip(rels.single_rels_additional_fields().iter())
        {
            self.buf.write_all(b", ")?;
            let rel_table: &Ciboulette2PostgresTableSettings =
                ciboulette_table_store.get(rel_key)?;
            let rel_table_cte: Ciboulette2PostgresTableSettings =
                rel_table.to_cte(Cow::Owned(format!("cte_{}_data", rel_table.name())))?;
            let rel_type: &CibouletteResourceType =
                main_type.get_relationship(&ciboulette_store, rel_key)?;
            self.write_table_info(&rel_table_cte)?;
            self.buf.write_all(b" AS (")?;
            self.gen_select_cte_single_rel(
                &rel_table,
                &rel_type,
                &query,
                &main_cte_data,
                &additional_fields.name(),
            )?;
            self.buf.write_all(b")")?;
            self.add_working_table(&rel_table, rel_table_cte);
        }
        Ok(())
    }
    fn gen_rel_additional_params(
        bucket: &'a CibouletteRelationshipBucket
    ) -> Result<[Ciboulette2SqlAdditionalField<'a>; 2], Ciboulette2SqlError> {
        Ok([
            Ciboulette2SqlAdditionalField::new(
                Ciboulette2PostgresTableField::new_owned(
                    Ciboulette2PostgresSafeIdent::try_from(bucket.from().as_str())?,
                    None,
                    None,
                ),
                Ciboulette2SqlAdditionalFieldType::Relationship,
            )?,
            Ciboulette2SqlAdditionalField::new(
                Ciboulette2PostgresTableField::new_owned(
                    Ciboulette2PostgresSafeIdent::try_from(bucket.to().as_str())?,
                    None,
                    None,
                ),
                Ciboulette2SqlAdditionalFieldType::Relationship,
            )?,
        ])
    }
    pub(crate) fn gen_select_multi_rel_routine(
        &mut self,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        query: &'a CibouletteQueryParameters<'a>,
        main_cte_data: &Ciboulette2PostgresTableSettings<'a>,
        rels: &Vec<Ciboulette2PostgresRelationships<'a>>,
    ) -> Result<(), Ciboulette2SqlError> {
        let rel_iter = rels.into_iter().peekable();
        for Ciboulette2PostgresRelationships {
            type_: rel_type,
            bucket,
            values: _rel_ids,
        } in rel_iter
        {
            self.buf.write_all(b", ")?;
            let additional_params = Self::gen_rel_additional_params(&bucket)?;
            let rel_table = ciboulette_table_store.get(rel_type.name().as_str())?;
            let rel_rel_table = ciboulette_table_store.get(bucket.resource().name().as_str())?;
            let rel_cte_rel_data = rel_rel_table
                .to_cte(Cow::Owned(format!("cte_rel_{}_rel_data", rel_table.name())))?;
            let rel_cte_data =
                rel_table.to_cte(Cow::Owned(format!("cte_rel_{}_data", rel_table.name())))?;
            self.write_table_info(&rel_cte_rel_data)?;
            self.buf.write_all(b" AS (")?;
            self.gen_select_cte_final(
                &rel_rel_table,
                &bucket.resource(),
                &query,
                &additional_params,
                query.include().contains(&bucket.resource()),
            )?;

            self.buf.write_all(b" INNER JOIN ")?;
            self.write_table_info(&main_cte_data)?;
            self.buf.write_all(b" ON ")?;
            self.compare_fields(
                &rel_rel_table,
                &Ciboulette2PostgresTableField::new_owned(
                    Ciboulette2PostgresSafeIdent::try_from(bucket.to().as_str())?,
                    None,
                    None,
                ),
                &main_cte_data,
                &Ciboulette2PostgresTableField::new_ref(main_cte_data.id().get_ident(), None, None),
            )?;
            self.buf.write_all(b"), ")?;
            self.write_table_info(&rel_cte_data)?;
            self.buf.write_all(b" AS (")?;
            self.gen_select_cte_final(
                &rel_table,
                &rel_type,
                &query,
                &[],
                query.include().contains(rel_type),
            )?;
            self.buf.write_all(b" INNER JOIN ")?;
            self.write_table_info(&rel_cte_rel_data)?;
            self.buf.write_all(b" ON ")?;
            self.compare_fields(
                &rel_cte_rel_data,
                &Ciboulette2PostgresTableField::new_ref(&additional_params[0].name(), None, None),
                &rel_table,
                &Ciboulette2PostgresTableField::new_ref(rel_table.id().get_ident(), None, None),
            )?;
            self.buf.write_all(b")")?;
            self.add_working_table(&rel_table, rel_cte_data);
            self.add_working_table(&rel_rel_table, rel_cte_rel_data);
        }
        Ok(())
    }
}
