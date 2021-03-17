use super::*;

#[inline]
fn extract_data<'a>(
    request: &'a CibouletteUpdateRequest<'a>
) -> Result<&'a CibouletteResource<'a, CibouletteResourceIdentifier<'a>>, Ciboulette2SqlError> {
    match request.data() {
        CibouletteUpdateRequestType::MainType(attr) => Ok(attr),
        CibouletteUpdateRequestType::Relationship(_) => {
            Err(Ciboulette2SqlError::UpdatingRelationships)
        }
    }
}

impl<'a> Ciboulette2PostgresBuilder<'a> {
    #[inline]
    fn gen_update_main_update(
        &mut self,
        request: &'a CibouletteUpdateRequest<'a>,
        main_table: &'a Ciboulette2PostgresTableSettings<'a>,
        main_update_cte: &Ciboulette2PostgresTableSettings<'a>,
        values: Vec<(&'a str, Ciboulette2SqlValue<'a>)>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(&main_update_cte)?;
        self.buf.write_all(b" AS (")?;
        self.gen_update_normal(&main_table, values, &request, true)?;
        self.buf.write_all(b"), ")?;
        Ok(())
    }

    #[inline]
    fn gen_update_main_update_data(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        request: &'a CibouletteUpdateRequest<'a>,
        main_update_cte: &Ciboulette2PostgresTableSettings<'a>,
        main_data_cte: &Ciboulette2PostgresTableSettings<'a>,
        rels: &Ciboulette2SqlQueryRels<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(&main_data_cte)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte_final(
            &state,
            &main_update_cte,
            &request.resource_type(),
            rels.single_rels_additional_fields().iter(),
            true,
        )?;
        self.buf.write_all(b")")?;
        Ok(())
    }

    #[inline]
    fn gen_update_rel_data_single(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        main_data_cte: &Ciboulette2PostgresTableSettings<'a>,
        rels: &Ciboulette2SqlQueryRels<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.gen_select_single_rel_routine(&state, &main_data_cte, rels)?;
        Ok(())
    }

    #[inline]
    fn gen_update_rel_data_multi(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        main_data_cte: &Ciboulette2PostgresTableSettings<'a>,
        multi_relationships: &Vec<Ciboulette2PostgresRelationships<'a>>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.gen_select_multi_rel_routine(&state, &main_data_cte, &multi_relationships)?;
        Ok(())
    }

    pub fn gen_update_main(
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        request: &'a CibouletteUpdateRequest<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let state = Ciboulette2PostgresBuilderState::new(
            ciboulette_store,
            ciboulette_table_store,
            request.path(),
            request.query(),
        )?;
        let mut se = Self::default();
        let main_attrs = extract_data(&request)?;
        let main_cte_update = state.main_table().to_cte(Cow::Owned(format!(
            "cte_{}_update",
            state.main_table().name()
        )))?;
        let main_cte_data = state.main_table().to_cte(Cow::Owned(format!(
            "cte_{}_data",
            state.main_table().name()
        )))?;
        let Ciboulette2PostgresMain {
            insert_values: main_update_values,
            single_relationships: main_single_relationships,
        } = crate::graph_walker::main::extract_fields(
            &ciboulette_store,
            state.main_type(),
            main_attrs.attributes(),
            main_attrs.relationships(),
            true,
        )?;
        let main_multi_relationships = crate::graph_walker::relationships::extract_fields(
            &ciboulette_store,
            state.main_type(),
            Some(main_attrs.relationships()),
        )?;
        let rels =
            Ciboulette2SqlQueryRels::new(main_single_relationships, main_multi_relationships)?;
        se.buf.write_all(b"WITH ")?;
        se.gen_update_main_update(
            &request,
            &state.main_table(),
            &main_cte_update,
            main_update_values,
        )?;
        se.gen_update_main_update_data(&state, &request, &main_cte_update, &main_cte_data, &rels)?;
        se.gen_update_rel_data_single(&state, &main_cte_data, &rels)?;
        se.gen_update_rel_data_multi(&state, &main_cte_data, &rels.multi_rels())?;
        se.buf.write_all(b" ")?;
        se.gen_cte_for_sort(
            &ciboulette_store,
            &ciboulette_table_store,
            &request.query(),
            &state.main_type(),
            &state.main_table(),
            &main_cte_data,
        )?;
        se.add_working_table(&state.main_table(), main_cte_data);
        // Aggregate every table using UNION ALL
        se.finish_request(state)?;
        Ok(se)
    }
}
