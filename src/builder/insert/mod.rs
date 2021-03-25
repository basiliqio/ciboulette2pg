use super::*;
pub mod main;
pub mod rel;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    /// Generate a SQL query to handle a `POST` request
    pub fn gen_insert(
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        request: &'a CibouletteCreateRequest<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let mut se = Self::default();
        check_insert_request(&request)?;
        let state = get_state!(&ciboulette_store, &ciboulette_table_store, &request)?;
        let (main_cte_insert, main_cte_data) = gen_insert_cte_tables(&state)?;

        let Ciboulette2PostgresMainResourceInformations {
            insert_values: main_inserts_values,
            single_relationships: main_single_relationships,
        } = crate::graph_walker::main::extract_fields_and_values(
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
        let rels = Ciboulette2SqlQueryRels::new(
            &state.main_type(),
            main_single_relationships,
            multi_rels,
        )?;
        se.buf.write_all(b"WITH ")?;
        se.write_main_table_inserts(&main_cte_insert, &state, main_inserts_values)?;
        se.write_main_table_select(&main_cte_data, &state, main_cte_insert, &rels)?;
        se.select_one_to_one_rels_routine(
            &state,
            state.main_type(),
            &main_cte_data,
            &rels,
            Ciboulette2PostgresBuilderState::is_needed_all,
        )?;
        se.inserts_handle_many_to_many_rels(&state, &main_cte_data, rels.multi_rels())?;
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
    fn write_main_table_select(
        &mut self,
        main_cte_data: &Ciboulette2PostgresTable<'a>,
        state: &Ciboulette2PostgresBuilderState<'a>,
        main_cte_insert: Ciboulette2PostgresTable<'a>,
        rels: &Ciboulette2SqlQueryRels<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(main_cte_data)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte_final(
            state,
            &main_cte_insert,
            &state.main_type(),
            rels.single_rels_additional_fields().iter(),
            true,
        )?;
        self.buf.write_all(b")")?;
        Ok(())
    }

    /// Write the main table `INSERT`
    fn write_main_table_inserts(
        &mut self,
        main_cte_insert: &Ciboulette2PostgresTable<'a>,
        state: &Ciboulette2PostgresBuilderState<'a>,
        main_inserts_values: Vec<(&str, Ciboulette2SqlValue<'a>)>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(main_cte_insert)?;
        self.buf.write_all(b" AS (")?;
        self.gen_insert_normal(state.main_table(), main_inserts_values, true)?;
        self.buf.write_all(b"),")?;
        Ok(())
    }
}

/// Gen the insert and data table for the query
fn gen_insert_cte_tables<'a>(
    state: &Ciboulette2PostgresBuilderState<'a>
) -> Result<(Ciboulette2PostgresTable<'a>, Ciboulette2PostgresTable<'a>), Ciboulette2SqlError> {
    let main_cte_insert = state.main_table().to_cte(Cow::Owned(format!(
        "cte_{}_insert",
        state.main_table().name()
    )))?;
    let main_cte_data = state.main_table().to_cte(Cow::Owned(format!(
        "cte_{}_data",
        state.main_table().name()
    )))?;
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
