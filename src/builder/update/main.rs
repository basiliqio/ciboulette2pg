use super::*;

/// Extract the data object from a request, fails if the request doesn't contain a main type
#[inline]
fn extract_data_object_from_update_request<'request>(
    request: &'request CibouletteUpdateRequest<'request>
) -> Result<
    &'request CibouletteResource<
        'request,
        MessyJsonObjectValue<'request>,
        CibouletteResourceIdentifier<'request>,
    >,
    Ciboulette2SqlError,
> {
    match request.data() {
        CibouletteUpdateRequestType::MainType(attr) => Ok(attr),
        CibouletteUpdateRequestType::Relationship(_) => {
            Err(Ciboulette2SqlError::ManyRelationshipDirectWrite)
        }
    }
}

impl<'request> Ciboulette2PostgresBuilder<'request> {
    /// Generate the main type update CTE
    #[inline]
    fn gen_update_main_update(
        &mut self,
        request: &'request CibouletteUpdateRequest<'request>,
        main_table: &Ciboulette2PostgresTable,
        main_update_cte: &Ciboulette2PostgresTable,
        values: Vec<(ArcStr, Ciboulette2SqlValue<'request>)>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(&main_update_cte)?;
        self.buf.write_all(b" AS (")?;
        self.gen_update_normal(&main_table, values, &request, true)?;
        self.buf.write_all(b"), ")?;
        Ok(())
    }

    /// Generate the main type data fetch CTE from the update CTE
    #[inline]
    fn gen_update_main_update_data<'store>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        request: &'request CibouletteUpdateRequest<'request>,
        main_update_cte: &Ciboulette2PostgresTable,
        main_data_cte: &Ciboulette2PostgresTable,
        rels: &[Ciboulette2SqlAdditionalField],
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(&main_data_cte)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte_final(
            &state,
            &main_update_cte,
            request.resource_type().clone(),
            None,
            rels.iter(),
            true,
        )?;
        self.buf.write_all(b")")?;
        Ok(())
    }

    /// Generate a SQL query to handle a `PATCH` request
    ///
    /// Update the main objects and its one-to-one relationships
    /// Fails if trying to update one-to-many relationships
    pub(crate) fn gen_update_main<'store>(
        ciboulette_store: &'store CibouletteStore,
        ciboulette_table_store: &'store Ciboulette2PostgresTableStore,
        request: &'request CibouletteUpdateRequest<'request>,
    ) -> Result<Self, Ciboulette2SqlError>
    where
        'store: 'request,
    {
        let state = get_state!(&ciboulette_store, &ciboulette_table_store, &request)?;
        let mut se = Self::default();
        let main_data = extract_data_object_from_update_request(&request)?;
        let (main_cte_update, main_cte_data) = Self::gen_update_cte_tables(&state.main_table())?;
        let Ciboulette2PostgresResourceInformations {
            values,
            single_relationships,
            single_relationships_additional_fields,
            multi_relationships,
        } = extract_data(
            &ciboulette_store,
            request.path().main_type().clone(),
            main_data.attributes(),
            main_data.relationships(),
            true,
        )?;
        se.buf.write_all(b"WITH ")?;
        se.gen_update_main_update(&request, &state.main_table(), &main_cte_update, values)?;
        se.gen_update_main_update_data(
            &state,
            &request,
            &main_cte_update,
            &main_cte_data,
            &single_relationships_additional_fields,
        )?;
        se.select_one_to_one_rels_routine(
            &state,
            &main_cte_data,
            &single_relationships,
            &single_relationships_additional_fields,
            Ciboulette2PostgresBuilderState::is_needed_all,
        )?;
        se.select_multi_rels_routine(
            &state,
            &main_cte_data,
            &multi_relationships,
            Ciboulette2PostgresBuilderState::is_needed_all,
        )?;
        se.buf.write_all(b" ")?;
        se.gen_cte_for_sort(&state, &main_cte_data)?;
        se.add_working_table(
            &state.main_table(),
            (main_cte_data, Ciboulette2PostgresResponseType::Object),
        );
        // Aggregate every table using UNION ALL
        se.finish_request(state)?;
        Ok(se)
    }
}
