use super::*;

fn extract_rels<'a>(
    request: &'a CibouletteUpdateRequest<'a>,
) -> Result<&'a CibouletteUpdateRelationship<'a>, Ciboulette2SqlError> {
    match request.data() {
        CibouletteUpdateRequestType::MainType(_) => Err(Ciboulette2SqlError::UpdatingMainObject),
        CibouletteUpdateRequestType::Relationship(rels) => Ok(rels),
    }
}

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub fn gen_update_rel_update(
        &mut self,
        request: &'a CibouletteUpdateRequest<'a>,
        main_table: &Ciboulette2PostgresTableSettings<'a>,
        main_cte_update: &Ciboulette2PostgresTableSettings<'a>,
        values: Vec<(&'a str, Ciboulette2SqlValue<'a>)>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(&main_cte_update)?;
        self.buf.write_all(b" AS (")?;
        self.gen_update_normal(&main_table, values, &request, true)?;
        self.buf.write_all(b"), ")?;
        Ok(())
    }

    pub fn gen_update_rel_data(
        &mut self,
        request: &'a CibouletteUpdateRequest<'a>,
        main_type: &'a CibouletteResourceType<'a>,
        main_cte_update: &Ciboulette2PostgresTableSettings<'a>,
        main_cte_data: &Ciboulette2PostgresTableSettings<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(&main_cte_data)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte_final(&main_cte_update, &main_type, &request.query(), true)?;
        self.buf.write_all(b")")?;
        Ok(())
    }

    pub fn gen_update_rel(
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        request: &'a CibouletteUpdateRequest<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let mut se = Self::default();
        let rels = extract_rels(&request)?;
        let main_type = request.resource_type();
        let main_table = ciboulette_table_store.get(main_type.name().as_str())?;
        let main_cte_update =
            main_table.to_cte(Cow::Owned(format!("cte_{}_update", main_table.name())))?;
        let main_cte_data =
            main_table.to_cte(Cow::Owned(format!("cte_{}_data", main_table.name())))?;
        let Ciboulette2PostgresMain {
            insert_values: rel_values,
            single_relationships,
        } = crate::graph_walker::relationships::gen_query_rel(
            &ciboulette_store,
            &request.resource_type(),
            &rels,
        )?;
        se.buf.write_all(b"WITH ")?;
        se.gen_update_rel_update(&request, &main_table, &main_cte_update, rel_values)?;
        se.gen_update_rel_data(
            &request,
            &request.resource_type(),
            &main_cte_update,
            &main_cte_data,
        )?;

        se.gen_select_single_rel_routine(
            &ciboulette_store,
            &ciboulette_table_store,
            request.query(),
            &main_type,
            &main_cte_update,
            single_relationships,
        )?;
        se.buf.write_all(b" ")?;
        let sorting_map = se.gen_cte_for_sort(
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
