use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    /// Generate a SQL query to handle a `SELECT` request
    pub fn gen_select(
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        request: &'a CibouletteReadRequest<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let mut se = Self::default();
        let state = get_state!(&ciboulette_store, &ciboulette_table_store, &request)?;
        let main_cte_data = state.main_table().to_cte(Cow::Owned(format!(
            "cte_{}_data",
            state.main_table().name()
        )))?;
        let rels = Self::get_relationships(&ciboulette_store, &state.main_type())?;

        se.buf.write_all(b"WITH \n")?;
        se.gen_main_select(&state, &main_cte_data, &rels)?;

        se.select_single_rels_routine(&state, &main_cte_data, &rels)?;
        se.select_multi_rels_routine(&state, &main_cte_data, &rels.multi_rels())?;
        se.gen_cte_for_sort(&state, &main_cte_data)?;
        se.add_working_table(
            &state.main_table(),
            (main_cte_data, CibouletteResponseRequiredType::Object),
        );
        // Aggregate every table using UNION ALL
        se.finish_request(state)?;
        Ok(se)
    }

    /// Insert `INNER JOIN`s and `WHERE` close into the query for selecting related object
    fn gen_matcher_for_related_select(
        &mut self,
        ciboulette_table_store: &Ciboulette2PostgresTableStore<'a>,
        left_type: &&CibouletteResourceType<'a>,
        right_type: &&CibouletteResourceType<'a>,
        state: &Ciboulette2PostgresBuilderState<'a>,
        id: &CibouletteId<'a>,
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
        state: &Ciboulette2PostgresBuilderState<'a>,
        id: &CibouletteId<'a>,
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
        state: &Ciboulette2PostgresBuilderState<'a>,
        main_cte_data: &Ciboulette2PostgresTableSettings<'a>,
        rels: &Ciboulette2SqlQueryRels<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(main_cte_data)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte_final(
            state,
            &state.main_table(),
            &state.main_type(),
            rels.single_rels_additional_fields().iter(),
            true,
        )?;
        match state.path() {
            CiboulettePath::TypeIdRelated(left_type, id, right_type) => self
                .gen_matcher_for_related_select(
                    state.table_store(),
                    left_type,
                    right_type,
                    state,
                    id,
                )?,
            CiboulettePath::TypeId(_, id) | CiboulettePath::TypeIdRelationship(_, id, _) => {
                self.gen_matcher_for_normal_select(state, id)?
            }
            _ => (),
        }
        self.buf.write_all(b")")?;
        Ok(())
    }
}
