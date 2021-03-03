use super::*;
use crate::graph_walker::creation::main::Ciboulette2PostgresMainInsert;
use crate::graph_walker::creation::relationships::Ciboulette2PostgresRelationshipsInsert;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    fn gen_rel_select(
        &mut self,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        request: &'a CibouletteCreateRequest<'a>,
        table_list: &mut Vec<Ciboulette2PostgresTableSettings<'a>>,
        main_cte_data: &Ciboulette2PostgresTableSettings<'a>,
        rels: Vec<Ciboulette2PostgresRelationshipsInsert<'a>>,
    ) -> Result<(), Ciboulette2SqlError> {
        let rel_iter = rels.into_iter().peekable();

        for Ciboulette2PostgresRelationshipsInsert {
            type_: rel_type,
            bucket,
            values: rel_ids,
        } in rel_iter
        {
            self.buf.write_all(b", ")?;
            let rel_table = ciboulette_table_store.get(rel_type.name().as_str())?;
            let rel_cte_id =
                rel_table.to_cte(Cow::Owned(format!("cte_rel_{}_id", rel_table.name())));
            let rel_cte_insert =
                rel_table.to_cte(Cow::Owned(format!("cte_rel_{}_insert", rel_table.name())));
            let rel_cte_rel_data =
                rel_table.to_cte(Cow::Owned(format!("cte_rel_{}_rel_data", rel_table.name())));
            let rel_cte_data =
                rel_table.to_cte(Cow::Owned(format!("cte_rel_{}_data", rel_table.name())));
            // "cte_rel_myrel_id"
            self.write_table_info(&rel_cte_id)?;
            // "cte_rel_myrel_id" AS (VALUES
            self.buf.write_all(b" AS (VALUES ")?;
            // "cte_rel_myrel_id" AS (VALUES ($0::type), ($1::type)
            self.gen_rel_values(rel_ids, rel_table.id_type())?;
            // "cte_rel_myrel_id" AS (VALUES ($0::type), ($1::type)),
            self.buf.write_all(b"), ")?;
            // "cte_rel_myrel_id" AS (VALUES ($0::type), ($1::type)), "cte_rel_myrel_insert"
            self.write_table_info(&rel_cte_insert)?;
            // "cte_rel_myrel_id" AS (VALUES ($0::type), ($1::type)), "cte_rel_myrel_insert" AS (
            self.buf.write_all(b" AS (")?;
            // "cte_rel_myrel_id" AS (VALUES ($0::type), ($1::type)), "cte_rel_myrel_insert" AS (insert_stmt)
            self.gen_rel_insert(
                &rel_table,
                bucket.from().as_str(),
                bucket.to().as_str(),
                &main_cte_data,
                &rel_cte_id,
            )?;
            self.buf.write_all(b"), ")?;
            // "cte_rel_myrel_id" AS (VALUES ($0::type), ($1::type)), "cte_rel_myrel_insert" AS (insert_stmt), "cte_rel_myrel_data"
            self.write_table_info(&rel_cte_rel_data)?;
            // "cte_rel_myrel_id" AS (VALUES ($0::type), ($1::type)), "cte_rel_myrel_insert" AS (insert_stmt), "cte_rel_myrel_rel_data" AS (
            self.buf.write_all(b" AS (")?;
            // "cte_rel_myrel_id" AS (VALUES ($0::type), ($1::type)), "cte_rel_myrel_insert" AS (insert_stmt), "cte_rel_myrel_rel_data" AS (select_stmt)
            self.gen_select_cte_final(&rel_cte_insert, &bucket.resource(), &request.query())?;
            self.buf.write_all(b"), ")?;
            // "cte_rel_myrel_id" AS (VALUES ($0::type), ($1::type)), "cte_rel_myrel_insert" AS (insert_stmt), "cte_rel_myrel_rel_data" AS (select_stmt), "cte_rel_myrel_data" AS (select_stmt)
            self.write_table_info(&rel_cte_data)?;
            self.buf.write_all(b" AS (")?;
            self.gen_select_cte_final(&rel_table, &rel_type, &request.query())?;
            self.buf.write_all(b" IN (SELECT \"id\" FROM ")?;
            self.write_table_info(&rel_cte_id)?;
            self.buf.write_all(b")")?;
            table_list.push(rel_cte_rel_data);
            table_list.push(rel_cte_data);
        }
        Ok(())
    }

    pub fn gen_update_main(
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        request: &'a CibouletteCreateRequest<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let mut se = Self::default();
        let mut table_list: Vec<Ciboulette2PostgresTableSettings<'_>> = Vec::with_capacity(128);
        let main_type = request.path().main_type();
        let main_table = ciboulette_table_store.get(main_type.name().as_str())?;
        let main_cte_insert =
            main_table.to_cte(Cow::Owned(format!("cte_{}_update", main_table.name())));
        let main_cte_data =
            main_table.to_cte(Cow::Owned(format!("cte_{}_data", main_table.name())));
        table_list.push(main_cte_data.clone());
        // WITH
        se.buf.write_all(b"WITH \n")?;
        // WITH "cte_main_insert"
        se.write_table_info(&main_cte_insert)?;
        // WITH "cte_main_insert" AS (
        se.buf.write_all(b" AS (")?;
        let Ciboulette2PostgresMainInsert {
            insert_values: main_inserts_values,
            single_relationships: main_single_relationships,
        } = crate::graph_walker::creation::main::gen_query_insert(&ciboulette_store, &request)?;
        // WITH "cte_main_insert" AS (insert_stmt)
        se.gen_insert_normal(&main_table, main_inserts_values, true)?;
        se.buf.write_all(b"), ")?;
        // WITH "cte_main_insert" AS (insert_stmt), "cte_main_data"
        se.write_table_info(&main_cte_data)?;
        se.buf.write_all(b" AS (")?;
        se.gen_select_cte_final(&main_cte_insert, &main_type, &request.query())?;
        // WITH "cte_main_insert" AS (insert_stmt), "cte_main_data" AS (select_stmt)
        se.buf.write_all(b")")?;
        let rels = crate::graph_walker::creation::relationships::gen_query_insert(
            &ciboulette_store,
            &request,
        )?;

        let main_single_relationships_iter = main_single_relationships.into_iter();
        for key in main_single_relationships_iter {
            se.buf.write_all(b", ")?;
            let rel_table = ciboulette_table_store.get(key)?;
            let rel_table_cte =
                rel_table.to_cte(Cow::Owned(format!("cte_{}_data", rel_table.name())));
            let rel_type = main_type.get_relationship(&ciboulette_store, key)?;
            se.write_table_info(&rel_table_cte)?;
            se.buf.write_all(b" AS (")?;
            se.gen_select_cte_single_rel(
                &rel_table,
                &rel_type,
                &request.query(),
                &main_cte_insert,
                key,
            )?;
            se.buf.write_all(b")")?;
            table_list.push(rel_table_cte);
        }

        // WITH "cte_main_insert" AS (insert_stmt), "cte_main_data" AS (select_stmt),
        // * handle every (one-to-many) relationships *
        se.gen_insert_rel_routine(
            &ciboulette_table_store,
            &request,
            &mut table_list,
            &main_cte_data,
            rels,
        )?;
        se.buf.write_all(b" ")?;
        // Aggregate every table using UNION ALL
        se.gen_union_select_all(&table_list)?;
        Ok(se)
    }
}
