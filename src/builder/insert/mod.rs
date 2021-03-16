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
        } = crate::graph_walker::main::extract_fields(
            &ciboulette_store,
            request.path().main_type(),
            request.data().attributes(),
            request.data().relationships(),
            false,
        )?;
        let multi_rels = crate::graph_walker::relationships::extract_fields(
            &ciboulette_store,
            &request.path().main_type(),
            Some(request.data().relationships()),
        )?;
        let rels = Ciboulette2SqlQueryRels::new(main_single_relationships, multi_rels)?;
        se.buf.write_all(b"WITH \n")?;
        se.write_table_info(&main_cte_insert)?;
        se.buf.write_all(b" AS (")?;
        se.gen_insert_normal(&main_table, main_inserts_values, true)?;
        se.buf.write_all(b"),")?;
        se.write_table_info(&main_cte_data)?;
        se.buf.write_all(b" AS (")?;
        se.gen_select_cte_final(
            &main_cte_insert,
            &main_type,
            &request.query(),
            &rels.single_rels_additional_fields(),
            true,
        )?;
        se.buf.write_all(b")")?;

        se.gen_select_single_rel_routine(
            &ciboulette_store,
            &ciboulette_table_store,
            request.query(),
            &main_type,
            &&main_cte_data,
            &rels,
        )?;
        se.gen_insert_rel_routine(
            &ciboulette_table_store,
            &request,
            &main_cte_data,
            rels.multi_rels(),
        )?;
        se.buf.write_all(b" ")?;
        se.add_working_table(&main_table, main_cte_data);
        // Aggregate every table using UNION ALL
        se.gen_union_select_all(
            &ciboulette_store,
            &ciboulette_table_store,
            &request.query(),
            &main_table,
        )?;
        Ok(se)
    }
}
