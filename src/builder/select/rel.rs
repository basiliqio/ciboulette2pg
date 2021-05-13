use super::*;

impl<'request> Ciboulette2PgBuilder<'request> {
    /// Select the relationships of the main data, including every included relationships.
    ///
    /// It can include nested include, including every intermediate tables
    pub(crate) fn select_rels<'store>(
        &mut self,
        state: &Ciboulette2PgBuilderState<'store, 'request>,
        main_cte_data: &Ciboulette2PgTable,
        inclusion_map: &BTreeMap<
            Vec<CibouletteResourceRelationshipDetails>,
            (Ciboulette2PgResponseType, Vec<CibouletteSortingElement>),
        >,
    ) -> Result<(), Ciboulette2PgError> {
        for (included_list, (response_type, sort_fields_el)) in inclusion_map {
            let mut current_table = main_cte_data.clone();
            let mut current_type = main_cte_data.ciboulette_type().clone();
            for (include_el_index, include_el) in included_list.iter().enumerate() {
                let next_table = state
                    .table_store()
                    .get(include_el.related_type().name().as_str())?;
                let next_table_cte =
                    next_table.to_cte(&mut *self, CIBOULETTE_REL_PREFIX, CIBOULETTE_DATA_SUFFIX)?;
                let working_table = match include_el.relation_option() {
                    CibouletteRelationshipOption::ManyToOne(_) => next_table.as_ref(),
                    _ => &next_table_cte,
                };
                self.buf.write_all(b", ")?;
                self.write_table_info(&next_table_cte)?;
                self.buf.write_all(b" AS (")?;
                let current_rel_chain = &included_list[0..=include_el_index];
                let relating_field = Ciboulette2PgRelatingField::new(
                    Ciboulette2PgTableField::from(current_table.id()),
                    current_table.clone(),
                    &current_rel_chain,
                    current_type,
                );
                let mut sort_additional_fields = Vec::with_capacity(sort_fields_el.len());
                for sorting_element in sort_fields_el {
                    sort_additional_fields.push(Ciboulette2PgAdditionalField::from_sorting_field(
                        sorting_element,
                        next_table.ciboulette_type().clone(),
                    )?);
                }
                let rels =
                    extract_data_no_body(&state.store(), next_table_cte.ciboulette_type().clone())?;
                self.gen_select_cte(
					state,
					&next_table,
					working_table.ciboulette_type().clone(),
					Some(relating_field),
					rels.single_relationships_additional_fields().iter().chain(sort_additional_fields.iter()),
					!matches!(include_el.relation_option(), CibouletteRelationshipOption::ManyToOne(x) if x.part_of_many_to_many().is_some()) && matches!(response_type, Ciboulette2PgResponseType::Object),
				)?;
                Self::gen_inner_join(
                    &mut self.buf,
                    state,
                    &current_table,
                    include_el,
                    Some(&next_table),
                )?;
                self.buf.write_all(b") ")?;
                self.add_working_table(
                    current_rel_chain.to_vec(),
                    next_table_cte.clone(),
                    *response_type,
                );
                current_table = next_table_cte;
                current_type = current_table.ciboulette_type().clone();
            }
        }
        Ok(())
    }
}
