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
    Ciboulette2PgError,
> {
    match request.data() {
        CibouletteUpdateRequestType::MainType(attr) => Ok(attr),
        CibouletteUpdateRequestType::Relationship(_) => {
            Err(Ciboulette2PgError::ManyRelationshipDirectWrite)
        }
    }
}

impl<'request> Ciboulette2PgBuilder<'request> {
    /// Generate the main type update CTE
    #[inline]
    fn gen_update_main_update(
        &mut self,
        request: &'request CibouletteUpdateRequest<'request>,
        main_table: &Ciboulette2PgTable,
        main_update_cte: &Ciboulette2PgTable,
        values: Vec<(ArcStr, Ciboulette2PgValue<'request>)>,
    ) -> Result<(), Ciboulette2PgError> {
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
        state: &Ciboulette2PgBuilderState<'store, 'request>,
        request: &'request CibouletteUpdateRequest<'request>,
        main_update_cte: &Ciboulette2PgTable,
        main_data_cte: &Ciboulette2PgTable,
        rels: &[Ciboulette2PgAdditionalField],
    ) -> Result<(), Ciboulette2PgError> {
        let sort_keys_mains = Self::gen_sort_key_for_main(state, main_data_cte)?;
        self.write_table_info(&main_data_cte)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte(
            &state,
            &main_update_cte,
            request.resource_type().clone(),
            None,
            rels.iter().chain(sort_keys_mains.iter()),
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
        ciboulette_table_store: &'store Ciboulette2PgTableStore,
        request: &'request CibouletteUpdateRequest<'request>,
    ) -> Result<Self, Ciboulette2PgError>
    where
        'store: 'request,
    {
        let state = get_state!(&ciboulette_store, &ciboulette_table_store, &request)?;
        let mut se = Self::default();
        let main_data = extract_data_object_from_update_request(&request)?;
        let (main_cte_update, main_cte_data) =
            Self::gen_update_cte_tables(&mut se, &state.main_table())?;
        let mut request_resource = extract_data_from_ciboulette_request(
            &ciboulette_store,
            request.path().main_type().clone(),
            main_data.attributes(),
            main_data.relationships(),
            true,
        )?;
        let values = request_resource.take_values();
        se.buf.write_all(b"WITH ")?;
        match values.is_empty() {
            true => se.gen_update_select_if_empty_value(&state, &main_cte_update, request)?,
            false => {
                se.gen_update_main_update(&request, &state.main_table(), &main_cte_update, values)?
            }
        };
        se.gen_update_main_update_data(
            &state,
            &request,
            &main_cte_update,
            &main_cte_data,
            request_resource.single_relationships_additional_fields(),
        )?;
        se.select_rels(&state, &main_cte_data, state.inclusion_map())?;
        se.buf.write_all(b" ")?;
        // Aggregate every table using UNION ALL
        se.finish_request(state, main_cte_data, false)?;
        Ok(se)
    }

    /// Select the value that should've been updated, when updating a record
    /// with no provided values to update
    fn gen_update_select_if_empty_value<'store>(
        &mut self,
        state: &Ciboulette2PgBuilderState<'store, 'request>,
        main_cte_update: &Ciboulette2PgTable,
        request: &'request CibouletteUpdateRequest<'request>,
    ) -> Result<(), Ciboulette2PgError>
    where
        'store: 'request,
    {
        self.write_table_info(main_cte_update)?;
        self.buf.write_all(b" AS (SELECT * FROM ")?;
        self.write_table_info(state.main_table())?;
        self.gen_matcher_for_normal_select(state, request.resource_id())?;
        self.buf.write_all(b"), ")?;
        Ok(())
    }
}
