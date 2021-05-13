use super::*;

impl<'request> Ciboulette2PgBuilder<'request> {
    /// Generate a SQL query to handle a `SELECT` request
    pub fn gen_select<'store>(
        ciboulette_store: &'store CibouletteStore,
        ciboulette_table_store: &'store Ciboulette2PgTableStore,
        request: &'request CibouletteReadRequest<'request>,
    ) -> Result<Self, Ciboulette2PgError>
    where
        'store: 'request,
    {
        let mut se = Self::default();
        let state: Ciboulette2PgBuilderState<'store, 'request> =
            get_state!(&ciboulette_store, &ciboulette_table_store, &request)?;
        let main_cte_data =
            state
                .main_table()
                .to_cte(&mut se, CIBOULETTE_EMPTY_IDENT, CIBOULETTE_DATA_SUFFIX)?;
        let rels = extract_data_no_body(&ciboulette_store, state.main_type().clone())?;

        se.buf.write_all(b"WITH \n")?;
        let skip_main = se.gen_main_select(
            &state,
            &main_cte_data,
            rels.single_relationships_additional_fields(),
        )?;
        // Aggregate every table using UNION ALL
        se.finish_request(state, main_cte_data, skip_main)?;
        Ok(se)
    }

    /// Insert `INNER JOIN`s and `WHERE` close into the query for selecting related object
    fn gen_matcher_for_related_select<'store>(
        &mut self,
        ciboulette_table_store: &Ciboulette2PgTableStore,
        left_type: Arc<CibouletteResourceType>,
        rel_details: &CibouletteResourceRelationshipDetails,
        state: &Ciboulette2PgBuilderState<'store, 'request>,
        id: &'request CibouletteIdSelector<'request>,
    ) -> Result<(), Ciboulette2PgError>
    where
        'store: 'request,
    {
        let left_table = ciboulette_table_store.get(left_type.name().as_str())?;
        Self::gen_inner_join(&mut self.buf, state, &left_table, &rel_details, None)?;
        self.buf.write_all(b" WHERE ")?;
        self.compare_pkey(&left_table, id)?;
        Ok(())
    }

    /// Generate the main `SELECT` of the query
    fn gen_main_select<'store>(
        &mut self,
        state: &Ciboulette2PgBuilderState<'store, 'request>,
        main_cte_data: &Ciboulette2PgTable,
        rels: &[Ciboulette2PgAdditionalField],
    ) -> Result<bool, Ciboulette2PgError>
    where
        'store: 'request,
    {
        self.write_table_info(&main_cte_data)?;
        self.buf.write_all(b" AS (")?;
        let skip_main = match state.path() {
            CiboulettePath::TypeIdRelationship(left_type, id, rel_details) => {
                self.gen_main_select_type_relationships(
                    state,
                    &main_cte_data,
                    left_type,
                    rel_details,
                    rels,
                    id,
                )?;
                true
            }
            CiboulettePath::TypeIdRelated(left_type, id, rel_details) => {
                self.gen_main_select_type_related(
                    state,
                    rels,
                    left_type,
                    rel_details,
                    id,
                    &main_cte_data,
                )?;
                false
            }
            CiboulettePath::TypeId(_, id) => {
                self.gen_main_select_type_id(state, rels, id, &main_cte_data)?;
                false
            }
            CiboulettePath::Type(_) => {
                self.gen_main_select_type(state, rels, &main_cte_data)?;
                false
            }
        };
        Ok(skip_main)
    }

    /// Generate the select for the main data relationships to the related data
    fn gen_main_select_type_relationships<'store>(
        &mut self,
        state: &Ciboulette2PgBuilderState<'store, 'request>,
        main_cte_data: &Ciboulette2PgTable,
        left_type: &Arc<CibouletteResourceType>,
        rel_details: &CibouletteResourceRelationshipDetails,
        rels: &[Ciboulette2PgAdditionalField],
        id: &'store CibouletteIdSelector,
    ) -> Result<(), Ciboulette2PgError>
    where
        'store: 'request,
    {
        let main_type_table = state.table_store().get(left_type.name().as_str())?.clone();
        let main_type_cte =
            main_type_table.to_cte(&mut *self, CIBOULETTE_REL_PREFIX, CIBOULETTE_DATA_SUFFIX)?;
        let sort_keys_mains = Self::gen_sort_key_for_main(state, &main_type_table)?;
        let rels_main_type = extract_data_no_body(state.store(), state.path().base_type().clone())?;
        self.gen_select_cte(
            state,
            &state.main_table(),
            state.main_type().clone(),
            Some(Ciboulette2PgRelatingField::new(
                Ciboulette2PgTableField::from(main_type_table.id()),
                (&*main_type_table).clone(),
                &[rel_details.clone()],
                rel_details.related_type().clone(),
            )),
            rels.iter().chain(sort_keys_mains.iter()),
            false,
        )?;
        self.gen_matcher_for_related_select(
            state.table_store(),
            left_type.clone(),
            rel_details,
            state,
            id,
        )?;
        self.buf.write_all(b"), ")?;
        self.write_table_info(&main_type_cte)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte(
            state,
            &main_type_table,
            state.path().base_type().clone(),
            None,
            rels_main_type
                .single_relationships_additional_fields()
                .iter(),
            false,
        )?;
        self.buf.write_all(b" WHERE ")?;
        self.compare_pkey(&*main_type_table, id)?;
        self.buf.write_all(b") ")?;
        // Kind of hack, but replace the including, marking every working table as unneeded
        // TODO find a better way
        let inclusion_map: BTreeMap<
            Vec<CibouletteResourceRelationshipDetails>,
            (Ciboulette2PgResponseType, Vec<CibouletteSortingElement>),
        > = state
            .inclusion_map()
            .iter()
            .map(|(k, (_, e))| (k.clone(), (Ciboulette2PgResponseType::None, e.clone())))
            .collect();
        self.select_rels(&state, &main_cte_data, &inclusion_map)?;
        match state.query().sorting().is_empty() {
            true => {
                self.write_table_final_select(&main_cte_data)?;
                self.buf.write_all(b")")?;
            }
            false => self.gen_cte_main_final_sorting(&state, &main_cte_data)?,
        }
        Ok(())
    }

    fn gen_main_select_type_related<'store>(
        &mut self,
        state: &Ciboulette2PgBuilderState<'store, 'request>,
        rels: &[Ciboulette2PgAdditionalField],
        left_type: &Arc<CibouletteResourceType>,
        rel_details: &CibouletteResourceRelationshipDetails,
        id: &'store CibouletteIdSelector,
        main_cte_data: &Ciboulette2PgTable,
    ) -> Result<(), Ciboulette2PgError>
    where
        'store: 'request,
    {
        let sort_keys_mains = Self::gen_sort_key_for_main(state, main_cte_data)?;
        self.gen_select_cte(
            state,
            &state.main_table(),
            state.main_type().clone(),
            None,
            rels.iter().chain(sort_keys_mains.iter()),
            true,
        )?;
        self.gen_matcher_for_related_select(
            state.table_store(),
            left_type.clone(),
            rel_details,
            state,
            id,
        )?;
        self.buf.write_all(b")")?;
        self.select_rels(&state, &main_cte_data, state.inclusion_map())?;
        Ok(())
    }

    fn gen_main_select_type<'store>(
        &mut self,
        state: &Ciboulette2PgBuilderState<'store, 'request>,
        rels: &[Ciboulette2PgAdditionalField],
        main_cte_data: &Ciboulette2PgTable,
    ) -> Result<(), Ciboulette2PgError>
    where
        'store: 'request,
    {
        let sort_keys_mains = Self::gen_sort_key_for_main(state, main_cte_data)?;
        self.gen_select_cte(
            state,
            &state.main_table(),
            state.main_type().clone(),
            None,
            rels.iter().chain(sort_keys_mains.iter()),
            true,
        )?;
        self.buf.write_all(b")")?;
        self.select_rels(&state, &main_cte_data, state.inclusion_map())?;
        Ok(())
    }

    fn gen_main_select_type_id<'store>(
        &mut self,
        state: &Ciboulette2PgBuilderState<'store, 'request>,
        rels: &[Ciboulette2PgAdditionalField],
        id: &'store CibouletteIdSelector,
        main_cte_data: &Ciboulette2PgTable,
    ) -> Result<(), Ciboulette2PgError>
    where
        'store: 'request,
    {
        let sort_keys_mains = Self::gen_sort_key_for_main(state, main_cte_data)?;
        self.gen_select_cte(
            state,
            &state.main_table(),
            state.main_type().clone(),
            None,
            rels.iter().chain(sort_keys_mains.iter()),
            true,
        )?;
        self.buf.write_all(b" WHERE ")?;
        self.compare_pkey(state.main_table(), id)?;
        self.buf.write_all(b") ")?;
        self.select_rels(&state, &main_cte_data, state.inclusion_map())?;
        Ok(())
    }
}
