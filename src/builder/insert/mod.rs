use super::*;
pub mod main;

impl<'request> Ciboulette2PgBuilder<'request> {
    /// Generate a SQL query to handle a `POST` request
    pub fn gen_insert<'store>(
        ciboulette_store: &'store CibouletteStore,
        ciboulette_table_store: &'store Ciboulette2PgTableStore,
        request: &'request CibouletteCreateRequest<'request>,
    ) -> Result<Self, Ciboulette2PgError>
    where
        'store: 'request,
    {
        let mut se = Self::default();
        check_insert_request(&request)?;
        let state = get_state!(&ciboulette_store, &ciboulette_table_store, &request)?;
        let (main_cte_insert, main_cte_data) = se.gen_insert_cte_tables(&state)?;

        let mut request_resource = extract_data_from_ciboulette_request(
            &ciboulette_store,
            request.path().main_type().clone(),
            request.data().attributes(),
            request.data().relationships(),
            true,
        )?;
        se.buf.write_all(b"WITH ")?;
        // Insert the data in the database
        se.write_main_table_inserts(&main_cte_insert, &state, request_resource.take_values())?;
        // Return the just inserted data
        se.write_main_table_select(
            &main_cte_data,
            &state,
            main_cte_insert,
            request_resource.single_relationships_additional_fields(),
        )?;
        // Select the relationships
        se.select_rels(&state, &main_cte_data, state.inclusion_map())?;
        se.buf.write_all(b" ")?;
        // Aggregate every table using UNION ALL
        se.finish_request(state, main_cte_data, false)?;
        Ok(se)
    }

    /// Write the main table `SELECT` after having inserted it
    fn write_main_table_select<'store>(
        &mut self,
        main_cte_data: &Ciboulette2PgTable,
        state: &Ciboulette2PgBuilderState<'store, 'request>,
        main_cte_insert: Ciboulette2PgTable,
        rels: &[Ciboulette2PgAdditionalField],
    ) -> Result<(), Ciboulette2PgError> {
        let sort_keys_mains = Self::gen_sort_key_for_main(state, main_cte_data)?;
        self.write_table_info(main_cte_data)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte(
            state,
            &main_cte_insert,
            state.main_type().clone(),
            None,
            rels.iter().chain(sort_keys_mains.iter()),
            true,
        )?;
        self.buf.write_all(b")")?;
        Ok(())
    }

    /// Write the main table `INSERT`
    fn write_main_table_inserts<'store>(
        &mut self,
        main_cte_insert: &Ciboulette2PgTable,
        state: &Ciboulette2PgBuilderState<'store, 'request>,
        main_inserts_values: Vec<(ArcStr, Ciboulette2PgValue<'request>)>,
    ) -> Result<(), Ciboulette2PgError> {
        self.write_table_info(main_cte_insert)?;
        self.buf.write_all(b" AS (")?;
        self.gen_insert_normal(state.main_table().as_ref(), main_inserts_values, true)?;
        self.buf.write_all(b"),")?;
        Ok(())
    }

    /// Gen the insert and data table for the query
    fn gen_insert_cte_tables(
        &mut self,
        state: &Ciboulette2PgBuilderState,
    ) -> Result<(Ciboulette2PgTable, Ciboulette2PgTable), Ciboulette2PgError> {
        let main_cte_insert = state.main_table().to_cte(
            &mut *self,
            CIBOULETTE_EMPTY_IDENT,
            CIBOULETTE_INSERT_SUFFIX,
        )?;
        let main_cte_data = state.main_table().to_cte(
            &mut *self,
            CIBOULETTE_EMPTY_IDENT,
            CIBOULETTE_DATA_SUFFIX,
        )?;
        Ok((main_cte_insert, main_cte_data))
    }
}

/// Check that the insert request is correct
fn check_insert_request(request: &CibouletteCreateRequest) -> Result<(), Ciboulette2PgError> {
    if request.data().identifier().id().is_some() {
        Err(Ciboulette2PgError::ProvidedIdOnInserts)
    } else if request.data().attributes().is_none() {
        Err(Ciboulette2PgError::MissingAttributes)
    } else {
        Ok(())
    }
}
