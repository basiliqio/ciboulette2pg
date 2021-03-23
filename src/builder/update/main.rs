use super::*;

/// Extract the data object from a request, fails if the request doesn't contain a main type
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
    /// Generate the main type update CTE
    #[inline]
    fn gen_update_main_update(
        &mut self,
        request: &'a CibouletteUpdateRequest<'a>,
        main_table: &'a Ciboulette2PostgresTable<'a>,
        main_update_cte: &Ciboulette2PostgresTable<'a>,
        values: Vec<(&'a str, Ciboulette2SqlValue<'a>)>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(&main_update_cte)?;
        self.buf.write_all(b" AS (")?;
        self.gen_update_normal(&main_table, values, &request, true)?;
        self.buf.write_all(b"), ")?;
        Ok(())
    }

    /// Generate the main type data fetch CTE from the update CTE
    #[inline]
    fn gen_update_main_update_data(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        request: &'a CibouletteUpdateRequest<'a>,
        main_update_cte: &Ciboulette2PostgresTable<'a>,
        main_data_cte: &Ciboulette2PostgresTable<'a>,
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

    /// Generate a SQL query to handle a `PATCH` request
    ///
    /// Update the main objects and its one-to-one relationships
    /// Fails if trying to update one-to-many relationships
    pub(crate) fn gen_update_main(
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        request: &'a CibouletteUpdateRequest<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let state = get_state!(&ciboulette_store, &ciboulette_table_store, &request)?;
        let mut se = Self::default();
        let main_attrs = extract_data(&request)?;
        let (main_cte_update, main_cte_data) = Self::gen_update_cte_tables(&state.main_table())?;
        let Ciboulette2PostgresMainResourceInformations {
            insert_values: main_update_values,
            single_relationships: main_single_relationships,
        } = crate::graph_walker::main::extract_fields_and_values(
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
        se.select_one_to_one_rels_routine(&state, &state.main_type(), &main_cte_data, &rels)?;
        se.select_multi_rels_routine(&state, &main_cte_data, &rels.multi_rels())?;
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
