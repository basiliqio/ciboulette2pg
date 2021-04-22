use itertools::Itertools;

use super::*;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    pub(crate) fn finish_request<'store>(
        &mut self,
        state: Ciboulette2PostgresBuilderState<'store, 'request>,
        main_table: Ciboulette2PostgresTable,
        skip_main: bool,
    ) -> Result<(), Ciboulette2SqlError> {
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
            if matches!(is_needed, Ciboulette2PostgresResponseType::None) {
                continue;
            } else {
                self.buf.write_all(b" UNION ALL ")?;
            }
            // SELECT * FROM
            self.write_table_final_select(table)?;
            self.buf.write_all(b")")?;
        }
        Ok(())
    }

    pub(crate) fn gen_cte_main_final_sorting<'store>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        main_table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        let (final_data_table, sorting_field_map) =
            self.gen_select_cte_final(&state, main_table)?;
        self.write_table_final_select(&final_data_table)?;
        self.buf.write_all(b" ORDER BY ")?;
        for (idx, sorting_el) in state.query().sorting().iter().enumerate() {
            let field_name = sorting_field_map
                .get(sorting_el)
                .cloned()
                .ok_or(Ciboulette2SqlError::UnknownError)?;
            if idx != 0 {
                self.buf.write_all(b", ")?;
            }
            self.insert_ident(
                &Ciboulette2PostgresTableField::new(field_name, None, None),
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

    pub(crate) fn write_table_final_select(
        &mut self,
        table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write_all(b"(SELECT ")?;
        Self::insert_ident_inner(
            &mut self.buf,
            &Ciboulette2PostgresTableField::new(CIBOULETTE_ID_IDENT, None, None),
            &table,
            None,
        )?;
        self.buf.write_all(b", ")?;
        Self::insert_ident_inner(
            &mut self.buf,
            &Ciboulette2PostgresTableField::new(CIBOULETTE_TYPE_IDENT, None, None),
            &table,
            None,
        )?;
        self.buf.write_all(b", ")?;
        Self::insert_ident_inner(
            &mut self.buf,
            &Ciboulette2PostgresTableField::new(CIBOULETTE_DATA_IDENT, None, None),
            &table,
            None,
        )?;
        self.buf.write_all(b", ")?;
        Self::insert_ident_inner(
            &mut self.buf,
            &Ciboulette2PostgresTableField::new(CIBOULETTE_RELATED_TYPE_IDENT, None, None),
            &table,
            None,
        )?;
        self.buf.write_all(b", ")?;
        Self::insert_ident_inner(
            &mut self.buf,
            &Ciboulette2PostgresTableField::new(CIBOULETTE_RELATED_ID_IDENT, None, None),
            &table,
            None,
        )?;
        self.buf.write_all(b" FROM ")?;
        Self::write_table_info_inner(&mut self.buf, &table)?;
        Ok(())
    }
    pub(crate) fn gen_select_cte<'store, 'b, I>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        table: &Ciboulette2PostgresTable,
        type_: Arc<CibouletteResourceType>,
        relating_field: Option<Ciboulette2PostgresRelatingField>,
        additional_fields: I,
        include: bool,
    ) -> Result<(), Ciboulette2SqlError>
    where
        I: Iterator<Item = &'b Ciboulette2SqlAdditionalField>,
    {
        self.buf.write_all(b"SELECT ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new(
                table.id().get_ident().clone(),
                Some(CIBOULETTE_ID_IDENT),
                Some(TEXT_IDENT),
            ),
            table,
        )?;
        self.buf.write_all(b", ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new(
                table.id().get_ident().clone(),
                Some(CIBOULETTE_MAIN_IDENTIFIER),
                None,
            ),
            table,
        )?;
        self.buf.write_all(b", ")?;
        match relating_field {
            Some(relating_field) => {
                self.insert_ident(relating_field.field(), relating_field.table())?;
                self.buf.write_all(b"::TEXT AS \"related_id\", ")?;
                self.insert_params(
                    Ciboulette2SqlValue::ArcStr(Some(type_.name().clone())),
                    relating_field.table(),
                )?;
                self.buf.write_all(b"::TEXT AS \"related_type\", ")?;
                self.insert_params(
                    Ciboulette2SqlValue::Text(Some(Cow::Owned(
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
                    Ciboulette2SqlValue::ArcStr(Some(type_.name().clone())),
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

    pub(crate) fn gen_select_cte_final<'store>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        table: &Ciboulette2PostgresTable,
    ) -> Result<
        (
            Ciboulette2PostgresTable,
            BTreeMap<CibouletteSortingElement, Ciboulette2PostgresSafeIdent>,
        ),
        Ciboulette2SqlError,
    > {
        let mut sort_fields: BTreeMap<CibouletteSortingElement, Ciboulette2PostgresSafeIdent> =
            BTreeMap::new();
        let res_table = Ciboulette2PostgresTable::new(
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
        for (rel_chain, (_, sorting_elements)) in state.inclusion_map() {
            let rel_chain_str = Ciboulette2PostgresSafeIdent::try_from(
                rel_chain.iter().map(|x| x.relation_alias()).join("_"),
            )?
            .add_modifier(Ciboulette2PostgresSafeIdentModifier::Prefix(
                CIBOULETTE_SORT_PREFIX,
            ));
            let current_table = self
                .working_tables()
                .get(rel_chain)
                .map(|(k, _)| k)
                .cloned()
                .unwrap_or_else(|| table.clone());
            for sorting_el in sorting_elements {
                let new_sorting_field = Ciboulette2PostgresSafeIdent::try_from(sorting_el.field())?;
                let old_sorting_field = new_sorting_field.clone().add_modifier(
                    Ciboulette2PostgresSafeIdentModifier::Prefix(CIBOULETTE_SORT_PREFIX),
                );
                let new_sorting_handle = rel_chain_str.clone().add_modifier(
                    Ciboulette2PostgresSafeIdentModifier::Suffix(new_sorting_field),
                );
                self.buf.write_all(b", ")?;
                self.insert_ident(
                    &Ciboulette2PostgresTableField::new(
                        old_sorting_field,
                        Some(new_sorting_handle.clone()),
                        None,
                    ),
                    &current_table,
                )?;
                sort_fields.insert(sorting_el.clone(), new_sorting_handle);
            }
        }
        self.buf.write_all(b" FROM ")?;
        self.write_table_info(table)?;
        for rel_chain in state.inclusion_map().keys() {
            let mut current_table = table.clone();
            for (idx, rel) in rel_chain.iter().enumerate() {
                let current_rel_chain = &rel_chain[0..=idx];
                let left_table = self
                    .working_tables()
                    .get(current_rel_chain)
                    .cloned()
                    .map(|(x, _)| x)
                    .ok_or(Ciboulette2SqlError::UnknownError)?;
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
}
