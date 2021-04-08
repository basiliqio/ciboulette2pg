use super::*;

impl<'store, 'request> Ciboulette2PostgresBuilder<'store, 'request>
where
    'store: 'request,
{
    /// Generate a SQL query to handle a `SELECT` request
    pub fn gen_select(
        ciboulette_store: &'store CibouletteStore<'store>,
        ciboulette_table_store: &'store Ciboulette2PostgresTableStore<'store>,
        request: &'request CibouletteReadRequest<'request, 'store>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let mut se = Self::default();
        let state: Ciboulette2PostgresBuilderState<'store, 'request> =
            get_state!(&ciboulette_store, &ciboulette_table_store, &request)?;
        let main_cte_data = state.main_table().to_cte(Cow::Owned(format!(
            "cte_{}_data",
            state.main_table().name()
        )))?;
        let rels = Self::get_relationships(&ciboulette_store, state.main_type().clone())?;

        se.buf.write_all(b"WITH \n")?;
        se.gen_main_select(&state, &main_cte_data, &rels)?;

        let is_needed_cb = match request.path() {
            CiboulettePath::TypeIdRelationship(_, _, _) => {
                Ciboulette2PostgresBuilderState::is_needed_all_for_relationships
            }
            _ => Ciboulette2PostgresBuilderState::is_needed_all,
        };
        se.select_one_to_one_rels_routine(
            &state,
            state.main_type().clone(),
            &main_cte_data,
            &rels,
            is_needed_cb,
        )?;
        se.select_multi_rels_routine(&state, &main_cte_data, &rels.multi_rels(), is_needed_cb)?;
        se.gen_cte_for_sort(&state, &main_cte_data)?;
        se.add_working_table(
            &state.main_table(),
            (main_cte_data, Ciboulette2PostgresResponseType::Object),
        );
        // Aggregate every table using UNION ALL
        se.finish_request(state)?;
        Ok(se)
    }

    /// Insert `INNER JOIN`s and `WHERE` close into the query for selecting related object
    fn gen_matcher_for_related_select(
        &mut self,
        ciboulette_table_store: &Ciboulette2PostgresTableStore<'store>,
        left_type: Arc<CibouletteResourceType<'store>>,
        right_type: Arc<CibouletteResourceType<'store>>,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        id: &CibouletteId<'request>,
    ) -> Result<(), Ciboulette2SqlError> {
        let left_table = ciboulette_table_store.get(left_type.name().as_str())?;
        let right_table = ciboulette_table_store.get(right_type.name().as_str())?;
        Self::gen_inner_join(&mut self.buf, state, &left_table, &right_table)?;
        self.buf.write_all(b" WHERE ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(left_table.id().get_ident(), None, None),
            &left_table,
        )?;
        self.buf.write_all(b" = ")?;
        self.insert_params(Ciboulette2SqlValue::from(id), &left_table)?;
        Ok(())
    }

    /// Insert `WHERE` close into the query for selecting main object or relationships
    fn gen_matcher_for_normal_select(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        id: &CibouletteId<'request>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write_all(b" WHERE ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(
                state.main_table().id().get_ident(),
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
    fn gen_main_select(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        main_cte_data: &Ciboulette2PostgresTable<'store>,
        rels: &Ciboulette2SqlQueryRels<'store, 'request>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(main_cte_data)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte_final(
            state,
            &state.main_table(),
            state.main_type().clone(),
            None,
            rels.single_rels_additional_fields().iter(),
            !matches!(state.path(), CiboulettePath::TypeIdRelationship(_, _, _)),
        )?;
        match state.path() {
            CiboulettePath::TypeIdRelationship(left_type, id, right_type)
            | CiboulettePath::TypeIdRelated(left_type, id, right_type) => self
                .gen_matcher_for_related_select(
                    state.table_store(),
                    left_type.clone(),
                    right_type.clone(),
                    state,
                    id,
                )?,
            CiboulettePath::TypeId(_, id) => self.gen_matcher_for_normal_select(state, id)?,
            _ => (),
        }
        self.buf.write_all(b")")?;
        Ok(())
    }
}
