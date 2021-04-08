use super::*;

//// Extract the relationships object from a request, fails if the request contains a main type
fn extract_rels<'store, 'request>(
    request: &'request CibouletteUpdateRequest<'request, 'store>
) -> Result<&'request CibouletteUpdateRelationship<'request, 'store>, Ciboulette2SqlError>
where
    'store: 'request,
{
    match request.data() {
        CibouletteUpdateRequestType::MainType(_) => Err(Ciboulette2SqlError::UpdatingMainObject),
        CibouletteUpdateRequestType::Relationship(rels) => Ok(rels),
    }
}

impl<'store, 'request> Ciboulette2PostgresBuilder<'store, 'request>
where
    'store: 'request,
{
    /// Generate the relationship update CTE
    pub fn gen_update_rel_update(
        &mut self,
        request: &'request CibouletteUpdateRequest<'request, 'store>,
        main_table: &Ciboulette2PostgresTable<'store>,
        main_cte_update: &Ciboulette2PostgresTable<'store>,
        values: Vec<(
            Ciboulette2PostgresStr<'store>,
            Ciboulette2SqlValue<'request>,
        )>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(&main_cte_update)?;
        self.buf.write_all(b" AS (")?;
        self.gen_update_normal(&main_table, values, &request, true)?;
        self.buf.write_all(b"), ")?;
        Ok(())
    }

    /// Generate the relationship data select from the relationship update CTE
    pub(crate) fn gen_update_rel_data(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        type_: Arc<CibouletteResourceType<'store>>,
        main_cte_update: &Ciboulette2PostgresTable<'store>,
        main_cte_data: &Ciboulette2PostgresTable<'store>,
        rels: &Ciboulette2SqlQueryRels<'store, 'request>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(&main_cte_data)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte_final(
            &state,
            &main_cte_update,
            type_.clone(),
            None,
            rels.single_rels_additional_fields().iter(),
            true,
        )?;
        self.buf.write_all(b")")?;
        Ok(())
    }

    /// Generate a SQL query to handle a `PATCH` request
    ///
    /// Update the relationships (one-to-one only) of an object
    /// Fails if trying to update one-to-many relationships
    pub(crate) fn gen_update_rel(
        ciboulette_store: &'store CibouletteStore<'store>,
        ciboulette_table_store: &'store Ciboulette2PostgresTableStore<'store>,
        request: &'request CibouletteUpdateRequest<'request, 'store>,
        main_type: Arc<CibouletteResourceType<'store>>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let main_table = ciboulette_table_store.get(&main_type.name().as_str())?;
        let mut se = Self::default();
        let state = get_state!(&ciboulette_store, &ciboulette_table_store, &request)?;
        let rels: &'request CibouletteUpdateRelationship<'request, 'store> =
            extract_rels(&request)?;
        let (main_cte_update, main_cte_data) = Self::gen_update_cte_tables(&main_table)?;
        let Ciboulette2PostgresMainResourceInformations {
            insert_values: rel_values,
            single_relationships,
        } = crate::graph_walker::relationships::extract_fields_rel(
            &ciboulette_store,
            request.resource_type().clone(),
            &rels,
        )?;
        let rels = Ciboulette2SqlQueryRels::new(main_type.clone(), single_relationships, vec![])?;
        se.buf.write_all(b"WITH ")?;
        se.gen_update_rel_update(&request, &main_table, &main_cte_update, rel_values)?;
        se.gen_update_rel_data(
            &state,
            request.resource_type().clone(),
            &main_cte_update,
            &main_cte_data,
            &rels,
        )?;
        se.select_one_to_one_rels_routine(
            &state,
            main_type.clone(),
            &main_cte_data,
            &rels,
            Ciboulette2PostgresBuilderState::is_needed_updating_relationships,
        )?;
        se.buf.write_all(b" ")?;
        se.gen_cte_for_sort(&state, &main_cte_data)?;
        se.add_working_table(
            &main_table,
            (main_cte_data, Ciboulette2PostgresResponseType::Object),
        );
        // Aggregate every table using UNION ALL
        se.finish_request(state)?;
        Ok(se)
    }
}
