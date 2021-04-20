use super::*;
pub mod main;
pub mod rel;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    /// Generate a SQL query to handle a `POST` request
    pub fn gen_insert<'store>(
        ciboulette_store: &'store CibouletteStore,
        ciboulette_table_store: &'store Ciboulette2PostgresTableStore,
        request: &'request CibouletteCreateRequest<'request>,
    ) -> Result<Self, Ciboulette2SqlError>
    where
        'store: 'request,
    {
        let mut se = Self::default();
        check_insert_request(&request)?;
        let state = get_state!(&ciboulette_store, &ciboulette_table_store, &request)?;
        let (main_cte_insert, main_cte_data) = gen_insert_cte_tables(&state)?;

        let Ciboulette2PostgresResourceInformations {
            values,
            single_relationships,
            single_relationships_additional_fields,
            multi_relationships,
        } = extract_data(
            &ciboulette_store,
            request.path().main_type().clone(),
            request.data().attributes(),
            request.data().relationships(),
            true,
        )?;
        se.buf.write_all(b"WITH ")?;
        se.write_main_table_inserts(&main_cte_insert, &state, values)?;
        se.write_main_table_select(
            &main_cte_data,
            &state,
            main_cte_insert,
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
        se.add_working_table(
            &state.main_table(),
            (main_cte_data, Ciboulette2PostgresResponseType::Object),
        );
        // Aggregate every table using UNION ALL
        se.finish_request(state)?;
        Ok(se)
    }

    /// Write the main table `SELECT` after having inserted it
    fn write_main_table_select<'store>(
        &mut self,
        main_cte_data: &Ciboulette2PostgresTable,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        main_cte_insert: Ciboulette2PostgresTable,
        rels: &[Ciboulette2SqlAdditionalField],
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(main_cte_data)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte_final(
            state,
            &main_cte_insert,
            state.main_type().clone(),
            None,
            rels.iter(),
            true,
        )?;
        self.buf.write_all(b")")?;
        Ok(())
    }

    /// Write the main table `INSERT`
    fn write_main_table_inserts<'store>(
        &mut self,
        main_cte_insert: &Ciboulette2PostgresTable,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        main_inserts_values: Vec<(ArcStr, Ciboulette2SqlValue<'request>)>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(main_cte_insert)?;
        self.buf.write_all(b" AS (")?;
        self.gen_insert_normal(state.main_table().as_ref(), main_inserts_values, true)?;
        self.buf.write_all(b"),")?;
        Ok(())
    }
}

/// Gen the insert and data table for the query
fn gen_insert_cte_tables(
    state: &Ciboulette2PostgresBuilderState
) -> Result<(Ciboulette2PostgresTable, Ciboulette2PostgresTable), Ciboulette2SqlError> {
    let main_cte_insert = state
        .main_table()
        .to_cte(CIBOULETTE_EMPTY_IDENT, CIBOULETTE_INSERT_SUFFIX)?;
    let main_cte_data = state
        .main_table()
        .to_cte(CIBOULETTE_EMPTY_IDENT, CIBOULETTE_DATA_SUFFIX)?;
    Ok((main_cte_insert, main_cte_data))
}

/// Check that the insert request is correct
fn check_insert_request(request: &CibouletteCreateRequest) -> Result<(), Ciboulette2SqlError> {
    if request.data().identifier().id().is_some() {
        Err(Ciboulette2SqlError::ProvidedIdOnInserts)
    } else if request.data().attributes().is_none() {
        Err(Ciboulette2SqlError::MissingAttributes)
    } else {
        Ok(())
    }
}
