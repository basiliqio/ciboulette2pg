use super::*;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    /// Generate a SQL query to handle a `SELECT` request
    pub fn gen_select<'store>(
        ciboulette_store: &'store CibouletteStore,
        ciboulette_table_store: &'store Ciboulette2PostgresTableStore,
        request: &'request CibouletteReadRequest<'request>,
    ) -> Result<Self, Ciboulette2SqlError>
    where
        'store: 'request,
    {
        let mut se = Self::default();
        let state: Ciboulette2PostgresBuilderState<'store, 'request> =
            get_state!(&ciboulette_store, &ciboulette_table_store, &request)?;
        let main_cte_data =
            state
                .main_table()
                .to_cte(&mut se, CIBOULETTE_EMPTY_IDENT, CIBOULETTE_DATA_SUFFIX)?;
        let rels = extract_data_no_body(&ciboulette_store, state.main_type().clone())?;

        se.buf.write_all(b"WITH \n")?;
        se.gen_main_select(
            &state,
            &main_cte_data,
            rels.single_relationships_additional_fields(),
        )?;
        // Aggregate every table using UNION ALL
        se.finish_request(state, main_cte_data, false)?;
        Ok(se)
    }

    /// Insert `INNER JOIN`s and `WHERE` close into the query for selecting related object
    fn gen_matcher_for_related_select<'store>(
        &mut self,
        ciboulette_table_store: &Ciboulette2PostgresTableStore,
        left_type: Arc<CibouletteResourceType>,
        rel_details: &CibouletteResourceRelationshipDetails,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        id: &CibouletteId<'request>,
    ) -> Result<(), Ciboulette2SqlError>
    where
        'store: 'request,
    {
        let left_table = ciboulette_table_store.get(left_type.name().as_str())?;
        Self::gen_inner_join(&mut self.buf, state, &left_table, &rel_details, None)?;
        self.buf.write_all(b" WHERE ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new(left_table.id().get_ident().clone(), None, None),
            &left_table,
        )?;
        self.buf.write_all(b" = ")?;
        self.insert_params(Ciboulette2SqlValue::from(id), &left_table)?;
        Ok(())
    }

    /// Insert `WHERE` close into the query for selecting main object or relationships
    fn gen_matcher_for_normal_select<'store>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        id: &CibouletteId<'request>,
    ) -> Result<(), Ciboulette2SqlError>
    where
        'store: 'request,
    {
        self.buf.write_all(b" WHERE ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new(
                state.main_table().id().get_ident().clone(),
                None,
                None,
            ),
            &state.main_table(),
        )?;
        self.buf.write_all(b" = ")?;
        self.insert_params(Ciboulette2SqlValue::from(id), &state.main_table())?;
        Ok(())
    }

    /// Generate the main `SELECT` of the query
    fn gen_main_select<'store>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        main_cte_data: &Ciboulette2PostgresTable,
        rels: &[Ciboulette2SqlAdditionalField],
    ) -> Result<(), Ciboulette2SqlError>
    where
        'store: 'request,
    {
        let sort_keys_mains = Self::gen_sort_key_for_main(state, main_cte_data)?;
        self.write_table_info(main_cte_data)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte(
            state,
            &state.main_table(),
            state.main_type().clone(),
            None,
            rels.iter().chain(sort_keys_mains.iter()),
            !matches!(state.path(), CiboulettePath::TypeIdRelationship(_, _, _)),
        )?;
        match state.path() {
            CiboulettePath::TypeIdRelationship(left_type, id, rel_details) => {
                self.gen_matcher_for_related_select(
                    state.table_store(),
                    left_type.clone(),
                    rel_details,
                    state,
                    id,
                )?;
                self.buf.write_all(b")")?;
            }
            CiboulettePath::TypeIdRelated(left_type, id, rel_details) => {
                self.gen_matcher_for_related_select(
                    state.table_store(),
                    left_type.clone(),
                    rel_details,
                    state,
                    id,
                )?;
                self.buf.write_all(b")")?;
                self.select_rels(&state, &main_cte_data, state.inclusion_map())?;
            }
            CiboulettePath::TypeId(_, id) => {
                self.gen_matcher_for_normal_select(state, id)?;
                self.buf.write_all(b") ")?;
                self.select_rels(&state, &main_cte_data, state.inclusion_map())?;
            }
            _ => {
                self.buf.write_all(b")")?;
                self.select_rels(&state, &main_cte_data, state.inclusion_map())?;
            }
        }
        Ok(())
    }
}
