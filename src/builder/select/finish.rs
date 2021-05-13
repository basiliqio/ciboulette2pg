use itertools::Itertools;

use super::*;

impl<'request> Ciboulette2PgBuilder<'request> {
    /// Finish an SQL query, selecting every working table in the state.
    ///
    /// If the `skip_main` parameter is used, the main table table is not handled
    pub(crate) fn finish_request<'store>(
        &mut self,
        state: Ciboulette2PgBuilderState<'store, 'request>,
        main_table: Ciboulette2PgTable,
        skip_main: bool,
    ) -> Result<(), Ciboulette2PgError> {
        if !skip_main {
            match state.query().sorting().is_empty() {
                false => {
                    self.gen_cte_main_final_sorting(&state, &main_table)?;
                }
                true => {
                    self.write_table_final_select(&main_table)?;
                    self.buf.write_all(b")")?;
                }
            };
        }

        for (table, is_needed) in std::mem::take(&mut self.working_tables).values() {
            if matches!(is_needed, Ciboulette2PgResponseType::None) {
                // if the table is not needed, skip it
                continue;
            } else {
                // Begin with an "UNION ALL" as the main table should've been handled before in any case
                self.buf.write_all(b" UNION ALL ")?;
            }
            // Select the working table
            self.write_table_final_select(table)?;
            self.buf.write_all(b")")?;
        }
        Ok(())
    }

    /// Handle the main table sorting, create a new cte just for sorting
    pub(crate) fn gen_cte_main_final_sorting<'store>(
        &mut self,
        state: &Ciboulette2PgBuilderState<'store, 'request>,
        main_table: &Ciboulette2PgTable,
    ) -> Result<(), Ciboulette2PgError> {
        // Select the final main data, removing duplicates
        let (final_data_table, sorting_field_map) =
            self.gen_select_cte_final(&state, main_table)?;
        // From now on, we're not writing CTE anymore
        // Writing the final select, that'll be part of the database response
        self.write_table_final_select(&final_data_table)?;
        self.buf.write_all(b" ORDER BY ")?;
        // For every sorting field, in the order specified by the request, sort the request
        for (idx, sorting_el) in state.query().sorting().iter().enumerate() {
            let field_name = sorting_field_map
                .get(sorting_el)
                .cloned()
                .ok_or(Ciboulette2PgError::UnknownError)?;
            if idx != 0 {
                self.buf.write_all(b", ")?;
            }
            self.insert_ident(
                &Ciboulette2PgTableField::new(
                    Ciboulette2PgSafeIdentSelector::Single(field_name),
                    None,
                    None,
                ),
                &final_data_table,
            )?;
            match sorting_el.direction() {
                CibouletteSortingDirection::Asc => self.buf.write_all(b" ASC")?,
                CibouletteSortingDirection::Desc => self.buf.write_all(b" DESC")?,
            };
        }
        self.buf.write_all(b")")?;
        Ok(())
    }

    /// Select a table that will be part of the database response
    ///
    /// The select is wrapped in parenthesis, the parent function has the responsability to add the matching closing
    /// parenthesis
    pub(crate) fn write_table_final_select(
        &mut self,
        table: &Ciboulette2PgTable,
    ) -> Result<(), Ciboulette2PgError> {
        self.buf.write_all(b"(SELECT ")?;
        Self::insert_ident_inner(
            &mut self.buf,
            &Ciboulette2PgTableField::new(
                Ciboulette2PgSafeIdentSelector::Single(CIBOULETTE_ID_IDENT),
                None,
                None,
            ),
            &table,
            None,
        )?;
        self.buf.write_all(b", ")?;
        Self::insert_ident_inner(
            &mut self.buf,
            &Ciboulette2PgTableField::new(
                Ciboulette2PgSafeIdentSelector::Single(CIBOULETTE_TYPE_IDENT),
                None,
                None,
            ),
            &table,
            None,
        )?;
        self.buf.write_all(b", ")?;
        Self::insert_ident_inner(
            &mut self.buf,
            &Ciboulette2PgTableField::new(
                Ciboulette2PgSafeIdentSelector::Single(CIBOULETTE_DATA_IDENT),
                None,
                None,
            ),
            &table,
            None,
        )?;
        self.buf.write_all(b", ")?;
        Self::insert_ident_inner(
            &mut self.buf,
            &Ciboulette2PgTableField::new(
                Ciboulette2PgSafeIdentSelector::Single(CIBOULETTE_RELATED_TYPE_IDENT),
                None,
                None,
            ),
            &table,
            None,
        )?;
        self.buf.write_all(b", ")?;
        Self::insert_ident_inner(
            &mut self.buf,
            &Ciboulette2PgTableField::new(
                Ciboulette2PgSafeIdentSelector::Single(CIBOULETTE_RELATED_ID_IDENT),
                None,
                None,
            ),
            &table,
            None,
        )?;
        self.buf.write_all(b" FROM ")?;
        Self::write_table_info_inner(&mut self.buf, &table)?;
        Ok(())
    }

    /// Generate a select CTE.
    ///
    /// It exposes common keys such as :
    /// - id
    /// - type
    /// - data
    /// - related_type
    /// - related_id
    pub(crate) fn gen_select_cte<'store, 'b, I>(
        &mut self,
        state: &Ciboulette2PgBuilderState<'store, 'request>,
        table: &Ciboulette2PgTable,
        type_: Arc<CibouletteResourceType>,
        relating_field: Option<Ciboulette2PgRelatingField>,
        additional_fields: I,
        include: bool,
    ) -> Result<(), Ciboulette2PgError>
    where
        I: Iterator<Item = &'b Ciboulette2PgAdditionalField>,
    {
        self.buf.write_all(b"SELECT ")?;
        self.select_main_id_pretty(&table)?;
        self.buf.write_all(b", ")?;
        self.select_main_id_raw(&table)?;
        self.buf.write_all(b", ")?;
        match relating_field {
            Some(relating_field) => {
                self.insert_ident(relating_field.field(), relating_field.table())?;
                self.buf.write_all(b"::TEXT AS \"related_id\", ")?;
                self.insert_params(
                    Ciboulette2PgValue::ArcStr(Some(
                        relating_field.table().ciboulette_type().name().clone(),
                    )),
                    relating_field.table(),
                )?;
                self.buf.write_all(b"::TEXT AS \"related_type\", ")?;
                self.insert_params(
                    Ciboulette2PgValue::Text(Some(Cow::Owned(
                        relating_field
                            .rel_chain()
                            .iter()
                            .map(|x| x.relation_alias())
                            .join("."),
                    ))), // If it's a relating field, the 'related_type' field contains the relationships alias, to identify it later
                    table,
                )?;
            }
            None => {
                self.buf
                    .write_all(b"NULL::TEXT AS \"related_id\", NULL::TEXT AS \"related_type\", ")?;
                self.insert_params(
                    Ciboulette2PgValue::ArcStr(Some(type_.name().clone())),
                    table,
                )?;
            }
        }
        self.buf.write_all(b"::TEXT AS \"type\", ")?;
        self.gen_json_builder(table, type_, state.query(), include)?;
        self.buf.write_all(b" AS \"data\"")?;
        self.handle_additionnal_params(&table, additional_fields)?;
        self.buf.write_all(b" FROM ")?;
        self.write_table_info(table)?;
        Ok(())
    }

    /// Create a final CTE for the main data
    ///
    /// Aggregating in a single place the final main data, removing duplicates row
    /// and joining the sort keys that can be used later to sort the main data
    pub(crate) fn gen_select_cte_final<'store>(
        &mut self,
        state: &Ciboulette2PgBuilderState<'store, 'request>,
        table: &Ciboulette2PgTable,
    ) -> Result<
        (
            Ciboulette2PgTable,
            BTreeMap<CibouletteSortingElement, Ciboulette2PgSafeIdent>,
        ),
        Ciboulette2PgError,
    > {
        let sort_fields: BTreeMap<CibouletteSortingElement, Ciboulette2PgSafeIdent>;

        let res_table = Ciboulette2PgTable::new(
            table.id().clone(),
            table.schema().clone(),
            CIBOULETTE_CTE_FINAL_MAIN_DATA,
            table.ciboulette_type().clone(),
        );
        self.buf.write_all(b", ")?;
        self.write_table_info(&res_table)?;
        self.buf.write_all(b" AS (SELECT DISTINCT ON (")?;
        self.write_table_info(&table)?;
        self.buf.write_all(b".\"")?;
        CIBOULETTE_MAIN_IDENTIFIER.to_writer(&mut self.buf)?;
        self.buf.write_all(b"\") ")?;
        self.write_table_info(&table)?;
        self.buf.write_all(b".* ")?;
        sort_fields = self.gen_select_cte_final_rel_inclusion(state, table)?;
        self.buf.write_all(b" FROM ")?;
        self.write_table_info(table)?;
        for (rel_chain, (_, sorting_elements)) in state.inclusion_map() {
            if sorting_elements.is_empty()
            // Skip table with no sorting element
            {
                continue;
            }
            let mut current_table = table.clone();
            for (idx, rel) in rel_chain.iter().enumerate() {
                let current_rel_chain = &rel_chain[0..=idx];
                let left_table = self
                    .working_tables()
                    .get(current_rel_chain)
                    .cloned()
                    .map(|(x, _)| x)
                    .ok_or(Ciboulette2PgError::UnknownError)?;
                Self::gen_left_join(&mut self.buf, &left_table, rel, &current_table)?;
                current_table = left_table;
            }
        }
        self.buf.write_all(b" ORDER BY ")?;
        self.write_table_info(table)?;
        self.buf.write_all(b".\"")?;
        CIBOULETTE_MAIN_IDENTIFIER.to_writer(&mut self.buf)?;
        self.buf.write_all(b"\") ")?;
        Ok((res_table, sort_fields))
    }

    /// Generate the final CTE Sorting map, mapping sorting element to the identifier used
    fn gen_select_cte_final_rel_inclusion(
        &mut self,
        state: &Ciboulette2PgBuilderState,
        table: &Ciboulette2PgTable,
    ) -> Result<BTreeMap<CibouletteSortingElement, Ciboulette2PgSafeIdent>, Ciboulette2PgError>
    {
        let mut sort_fields: BTreeMap<CibouletteSortingElement, Ciboulette2PgSafeIdent> =
            BTreeMap::new();
        for (rel_chain, (_, sorting_elements)) in state.inclusion_map() {
            let rel_chain_str = Ciboulette2PgSafeIdent::try_from(
                rel_chain.iter().map(|x| x.relation_alias()).join("_"),
            )?
            .add_modifier(Ciboulette2PgSafeIdentModifier::Prefix(
                CIBOULETTE_SORT_PREFIX,
            ));
            let current_table = self
                .working_tables()
                .get(rel_chain)
                .map(|(k, _)| k)
                .cloned()
                .unwrap_or_else(|| table.clone());
            for sorting_el in sorting_elements {
                let new_sorting_field = Ciboulette2PgSafeIdent::try_from(sorting_el.field())?;
                let old_sorting_field =
                    new_sorting_field
                        .clone()
                        .add_modifier(Ciboulette2PgSafeIdentModifier::Prefix(
                            CIBOULETTE_SORT_PREFIX,
                        ));
                let new_sorting_handle = rel_chain_str
                    .clone()
                    .add_modifier(Ciboulette2PgSafeIdentModifier::Suffix(new_sorting_field));
                self.buf.write_all(b", ")?;
                self.insert_ident(
                    &Ciboulette2PgTableField::new(
                        Ciboulette2PgSafeIdentSelector::Single(old_sorting_field),
                        Some(new_sorting_handle.clone()),
                        None,
                    ),
                    &current_table,
                )?;
                sort_fields.insert(sorting_el.clone(), new_sorting_handle);
            }
        }
        Ok(sort_fields)
    }
}
