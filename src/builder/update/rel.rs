use super::*;

fn extract_rels<'a>(
    request: &'a CibouletteUpdateRequest<'a>
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
        state: &Ciboulette2PostgresBuilderState<'a>,
        type_: &'a CibouletteResourceType<'a>,
        main_cte_update: &Ciboulette2PostgresTableSettings<'a>,
        main_cte_data: &Ciboulette2PostgresTableSettings<'a>,
        rels: &Ciboulette2SqlQueryRels<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(&main_cte_data)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte_final(
            &state,
            &main_cte_update,
            &type_,
            rels.single_rels_additional_fields().iter(),
            true,
        )?;
        self.buf.write_all(b")")?;
        Ok(())
    }

    pub fn gen_update_rel(
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        request: &'a CibouletteUpdateRequest<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let mut se = Self::default();
        let state = Ciboulette2PostgresBuilderState::new(
            ciboulette_store,
            ciboulette_table_store,
            request.path(),
            request.query(),
            request.expected_response_type(),
        )?;

        let rels = extract_rels(&request)?;
        let main_cte_update = state.main_table().to_cte(Cow::Owned(format!(
            "cte_{}_update",
            state.main_table().name()
        )))?;
        let main_cte_data = state.main_table().to_cte(Cow::Owned(format!(
            "cte_{}_data",
            state.main_table().name()
        )))?;
        let Ciboulette2PostgresMain {
            insert_values: rel_values,
            single_relationships,
        } = crate::graph_walker::relationships::extract_fields_rel(
            &ciboulette_store,
            &request.resource_type(),
            &rels,
        )?;
        let rels = Ciboulette2SqlQueryRels::new(single_relationships, vec![])?;
        se.buf.write_all(b"WITH ")?;
        se.gen_update_rel_update(&request, &state.main_table(), &main_cte_update, rel_values)?;
        se.gen_update_rel_data(
            &state,
            &request.resource_type(),
            &main_cte_update,
            &main_cte_data,
            &rels,
        )?;

        se.select_single_rels_routine(&state, &main_cte_data, &rels)?;
        se.buf.write_all(b" ")?;
        se.gen_cte_for_sort(&state, &main_cte_data)?;
        se.add_working_table(
            &state.main_table(),
            (main_cte_data, CibouletteResponseRequiredType::Object),
        );
        // Aggregate every table using UNION ALL
        se.finish_request(state)?;
        Ok(se)
    }
}
