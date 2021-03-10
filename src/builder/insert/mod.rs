use super::*;
pub mod main;
pub mod rel;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub fn gen_insert(
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        request: &'a CibouletteCreateRequest<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let mut se = Self::default();
        let main_type = request.path().main_type();
        let main_table = ciboulette_table_store.get(main_type.name().as_str())?;
        let main_cte_insert =
            main_table.to_cte(Cow::Owned(format!("cte_{}_insert", main_table.name())))?;
        let main_cte_data =
            main_table.to_cte(Cow::Owned(format!("cte_{}_data", main_table.name())))?;

        let Ciboulette2PostgresMain {
            insert_values: main_inserts_values,
            single_relationships: main_single_relationships,
        } = crate::graph_walker::main::gen_query(
            &ciboulette_store,
            request.path().main_type(),
            request.data().attributes(),
            Some(request.data().relationships()),
            false,
        )?;
        let rels = crate::graph_walker::relationships::gen_query(
            &ciboulette_store,
            &request.path().main_type(),
            Some(request.data().relationships()),
        )?;
        // WITH
        se.buf.write_all(b"WITH \n")?;
        // WITH "cte_main_insert"
        se.write_table_info(&main_cte_insert)?;
        // WITH "cte_main_insert" AS (
        se.buf.write_all(b" AS (")?;
        // WITH "cte_main_insert" AS (insert_stmt)
        se.gen_insert_normal(&main_table, main_inserts_values, true)?;
        se.buf.write_all(b")")?;

        se.gen_select_single_rel_routine(
            &ciboulette_store,
            &ciboulette_table_store,
            request.query(),
            &main_type,
            &main_cte_insert,
            main_single_relationships,
        )?;

        // WITH "cte_main_insert" AS (insert_stmt), "cte_main_data" AS (select_stmt),
        // * handle every (one-to-many) relationships *
        se.gen_insert_rel_routine(&ciboulette_table_store, &request, &main_cte_data, rels)?;
        se.buf.write_all(b", ")?;
        // WITH "cte_main_insert" AS (insert_stmt), "cte_main_data"
        se.write_table_info(&main_cte_data)?;
        se.buf.write_all(b" AS (")?;
        se.gen_select_cte_final(&main_cte_insert, &main_type, &request.query(), true)?;
        // WITH "cte_main_insert" AS (insert_stmt), "cte_main_data" AS (select_stmt)
        se.buf.write_all(b") ")?;

        let sorting_map: HashMap<&CibouletteResourceType, Vec<&CibouletteSortingElement>> = se
            .gen_cte_for_sort(
                &ciboulette_store,
                &ciboulette_table_store,
                request.query(),
                &main_type,
                &main_table,
                &main_cte_data,
            )?;
        se.included_tables.insert(&main_table, main_cte_data);
        // Aggregate every table using UNION ALL
        se.gen_union_select_all(&ciboulette_table_store, &sorting_map)?;
        Ok(se)
    }
}