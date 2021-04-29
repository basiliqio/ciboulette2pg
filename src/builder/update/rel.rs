use super::*;

//// Extract the relationships object from a request, fails if the request contains a main type
fn extract_rels<'request>(
    request: &'request CibouletteUpdateRequest<'request>
) -> Result<&'request CibouletteUpdateRelationshipBody<'request>, Ciboulette2SqlError> {
    match request.data() {
        CibouletteUpdateRequestType::MainType(_) => Err(Ciboulette2SqlError::UpdatingMainObject),
        CibouletteUpdateRequestType::Relationship(rels) => Ok(rels),
    }
}

impl<'request> Ciboulette2PostgresBuilder<'request> {
    /// Generate the relationship update CTE
    pub fn gen_update_rel_update(
        &mut self,
        request: &'request CibouletteUpdateRequest<'request>,
        main_table: &Ciboulette2PostgresTable,
        main_cte_update: &Ciboulette2PostgresTable,
        values: Vec<(ArcStr, Ciboulette2SqlValue<'request>)>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(&main_cte_update)?;
        self.buf.write_all(b" AS (")?;
        self.gen_update_normal(&main_table, values, &request, true)?;
        self.buf.write_all(b"), ")?;
        Ok(())
    }

    /// Generate the relationship data select from the relationship update CTE
    pub(crate) fn gen_update_rel_data<'store>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        type_: Arc<CibouletteResourceType>,
        main_cte_update: &Ciboulette2PostgresTable,
        main_cte_data: &Ciboulette2PostgresTable,
        rels: &[Ciboulette2SqlAdditionalField],
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(&main_cte_data)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte(&state, &main_cte_update, type_, None, rels.iter(), true)?;
        self.buf.write_all(b")")?;
        Ok(())
    }

    /// Generate a SQL query to handle a `PATCH` request
    ///
    /// Update the relationships (one-to-one only) of an object
    /// Fails if trying to update one-to-many relationships
    pub(crate) fn gen_update_rel<'store>(
        ciboulette_store: &'store CibouletteStore,
        ciboulette_table_store: &'store Ciboulette2PostgresTableStore,
        request: &'request CibouletteUpdateRequest<'request>,
        main_type: Arc<CibouletteResourceType>,
        rel_details: &CibouletteResourceRelationshipDetails,
    ) -> Result<Self, Ciboulette2SqlError>
    where
        'store: 'request,
    {
        let main_table = ciboulette_table_store.get(&main_type.name().as_str())?;
        let mut se = Self::default();
        let state = get_state!(&ciboulette_store, &ciboulette_table_store, &request)?;
        let rels: &'request CibouletteUpdateRelationshipBody<'request> = extract_rels(&request)?;
        let (main_cte_update, main_cte_data) = Self::gen_update_cte_tables(&mut se, &main_table)?;
        let Ciboulette2PostgresResourceInformations {
            values,
            single_relationships,
            single_relationships_additional_fields,
            multi_relationships,
        } = extract_data_rels(
            &ciboulette_store,
            request.path().base_type().clone(),
            rel_details.relation_alias(),
            rels.value(),
        )?;
        se.buf.write_all(b"WITH ")?;
        se.gen_update_rel_update(&request, &main_table, &main_cte_update, values)?;
        se.gen_update_rel_data(
            &state,
            request.resource_type().clone(),
            &main_cte_update,
            &main_cte_data,
            &single_relationships_additional_fields,
        )?;
        let mut inclusion_map: BTreeMap<
            Vec<CibouletteResourceRelationshipDetails>,
            (
                Ciboulette2PostgresResponseType,
                Vec<CibouletteSortingElement>,
            ),
        > = BTreeMap::default();
        inclusion_map.insert(
            vec![rel_details.clone()],
            (
                Ciboulette2PostgresResponseType::Id,
                state
                    .inclusion_map()
                    .get(&vec![])
                    .map(|(_, x)| x)
                    .cloned()
                    .unwrap_or_default(),
            ),
        );
        se.select_rels(&state, &main_cte_data, &inclusion_map)?;
        // Aggregate every table using UNION ALL
        let rel_table = se
            .add_working_table(
                vec![rel_details.clone()],
                main_cte_data.clone(),
                Ciboulette2PostgresResponseType::Id,
            )
            .map(|(x, _)| x)
            .ok_or(Ciboulette2SqlError::UnknownError)?;
        match state.query().sorting().is_empty() {
            true => {
                se.write_table_final_select(&rel_table)?;
                se.buf.write_all(b")")?;
            }
            false => se.gen_cte_main_final_sorting(&state, &rel_table)?,
        }
        se.finish_request(state, main_cte_data, true)?;
        Ok(se)
    }
}
