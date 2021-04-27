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
    pub(crate) fn gen_matcher_for_normal_select_inner<'store>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        id: &CibouletteId<'request>,
        main_table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError>
    where
        'store: 'request,
    {
        self.buf.write_all(b" WHERE ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new(main_table.id().get_ident().clone(), None, None),
            &main_table,
        )?;
        self.buf.write_all(b" = ")?;
        self.insert_params(Ciboulette2SqlValue::from(id), &main_table)?;
        Ok(())
    }

    /// Insert `WHERE` close into the query for selecting main object or relationships
    pub(crate) fn gen_matcher_for_normal_select<'store>(
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
    ) -> Result<bool, Ciboulette2SqlError>
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

    fn gen_main_select_type_relationships<'store>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        main_cte_data: &Ciboulette2PostgresTable,
        left_type: &Arc<CibouletteResourceType>,
        rel_details: &CibouletteResourceRelationshipDetails,
        rels: &[Ciboulette2SqlAdditionalField],
        id: &'store CibouletteId,
    ) -> Result<(), Ciboulette2SqlError>
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
            Some(Ciboulette2PostgresRelatingField::new(
                Ciboulette2PostgresTableField {
                    name: Ciboulette2PostgresSafeIdent::from(main_type_table.id().get_ident()),
                    alias: None,
                    cast: None,
                },
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
        self.gen_matcher_for_normal_select_inner(state, id, &*main_type_table)?;
        self.buf.write_all(b") ")?;
        let inclusion_map: BTreeMap<
            Vec<CibouletteResourceRelationshipDetails>,
            (
                Ciboulette2PostgresResponseType,
                Vec<CibouletteSortingElement>,
            ),
        > = state
            .inclusion_map()
            .iter()
            .map(|(k, (_, e))| {
                (
                    k.clone(),
                    (Ciboulette2PostgresResponseType::None, e.clone()),
                )
            })
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
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        rels: &[Ciboulette2SqlAdditionalField],
        left_type: &Arc<CibouletteResourceType>,
        rel_details: &CibouletteResourceRelationshipDetails,
        id: &'store CibouletteId,
        main_cte_data: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError>
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
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        rels: &[Ciboulette2SqlAdditionalField],
        main_cte_data: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError>
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
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        rels: &[Ciboulette2SqlAdditionalField],
        id: &'store CibouletteId,
        main_cte_data: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError>
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
        self.gen_matcher_for_normal_select(state, id)?;
        self.buf.write_all(b") ")?;
        self.select_rels(&state, &main_cte_data, state.inclusion_map())?;
        Ok(())
    }
}
