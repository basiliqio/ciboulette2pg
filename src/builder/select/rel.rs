use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(crate) fn gen_select_single_rel_routine(
        &mut self,
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        query: &'a CibouletteQueryParameters<'a>,
        main_type: &'a CibouletteResourceType<'a>,
        main_cte_data: &Ciboulette2PostgresTableSettings<'a>,
        rels: Vec<&'a str>,
    ) -> Result<(), Ciboulette2SqlError> {
        for key in rels.into_iter() {
            self.buf.write_all(b", ")?;
            let rel_table: &Ciboulette2PostgresTableSettings = ciboulette_table_store.get(key)?;
            let rel_table_cte: Ciboulette2PostgresTableSettings =
                rel_table.to_cte(Cow::Owned(format!("cte_{}_data", rel_table.name())))?;
            let rel_type: &CibouletteResourceType =
                main_type.get_relationship(&ciboulette_store, key)?;
            self.write_table_info(&rel_table_cte)?;
            self.buf.write_all(b" AS (")?;
            self.gen_select_cte_single_rel(
                &rel_table,
                &rel_type,
                &query,
                &main_cte_data,
                &Ciboulette2PostgresSafeIdent::try_from(key)?,
            )?;
            self.buf.write_all(b")")?;
            self.add_working_table(&rel_table, rel_table_cte);
        }
        Ok(())
    }
    fn gen_rel_additional_params(
        bucket: &'a CibouletteRelationshipBucket
    ) -> Result<[Ciboulette2SqlAdditonalField<'a>; 2], Ciboulette2SqlError> {
        Ok([
            Ciboulette2SqlAdditonalField::new(
                Ciboulette2PostgresTableField::new_owned(
                    Ciboulette2PostgresSafeIdent::try_from(bucket.from().as_str())?,
                    None,
                    None,
                ),
                Ciboulette2SqlAdditonalFieldType::Relationship,
            ),
            Ciboulette2SqlAdditonalField::new(
                Ciboulette2PostgresTableField::new_owned(
                    Ciboulette2PostgresSafeIdent::try_from(bucket.to().as_str())?,
                    None,
                    None,
                ),
                Ciboulette2SqlAdditonalFieldType::Relationship,
            ),
        ])
    }
    pub(crate) fn gen_select_multi_rel_routine(
        &mut self,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        query: &'a CibouletteQueryParameters<'a>,
        main_cte_data: &Ciboulette2PostgresTableSettings<'a>,
        rels: Vec<Ciboulette2PostgresRelationships<'a>>,
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
            // "" AS (select_stmt WHERE
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
                &Ciboulette2PostgresTableField::new_ref(main_cte_data.id_name(), None, None),
            )?;
            // "cte_rel_myrel_rel_data" AS (select_stmt WHERE "schema"."my_rel_rel"."to" = "cte_main_data"."myid"),
            self.buf.write_all(b"), ")?;
            self.write_table_info(&rel_cte_data)?;
            self.buf.write_all(b" AS (")?;
            // "cte_rel_myrel_rel_data" AS (select_stmt WHERE "schema"."my_rel_rel"."to" = "cte_main_data"."myid"), "cte_rel_myrel_data" AS (select_stmt)
            self.gen_select_cte_final(
                &rel_table,
                &rel_type,
                &query,
                &[],
                query.include().contains(&rel_type),
            )?;
            self.buf.write_all(b" WHERE ")?;
            // "cte_rel_myrel_rel_data" AS (select_stmt WHERE "schema"."my_rel_rel"."to" = "cte_main_data"."myid"), "cte_rel_myrel_data" AS (select_stmt) WHERE "schema"."rel_table"."id" IN (SELECT \"id\" FROM
            self.insert_ident(
                &Ciboulette2PostgresTableField::new_ref(rel_table.id_name(), None, None),
                &rel_table,
            )?;
            // "cte_rel_myrel_rel_data" AS (select_stmt WHERE "schema"."my_rel_rel"."to" = "cte_main_data"."myid"), "cte_rel_myrel_data" AS (select_stmt) WHERE "schema"."rel_table"."id" IN (SELECT \"id\" FROM
            self.buf.write_all(b" IN (SELECT ")?;
            self.insert_ident(
                &Ciboulette2PostgresTableField::new_owned(
                    Ciboulette2PostgresSafeIdent::try_from(bucket.from().as_str())?,
                    None,
                    None,
                ),
                &rel_cte_rel_data,
            )?;
            self.buf.write_all(b" FROM ")?;
            self.write_table_info(&rel_cte_rel_data)?;
            // "cte_rel_myrel_rel_data" AS (select_stmt WHERE "schema"."my_rel_rel"."to" = "cte_main_data"."myid"), "cte_rel_myrel_data" AS (select_stmt) WHERE "schema"."rel_table"."id" IN (SELECT \"id\" FROM "cte_rel_myrel_id")
            self.buf.write_all(b"))")?;
            self.add_working_table(&rel_table, rel_cte_data);
            self.add_working_table(&rel_rel_table, rel_cte_rel_data);
        }
        Ok(())
    }
}
