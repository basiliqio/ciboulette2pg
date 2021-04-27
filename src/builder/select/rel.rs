use super::*;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    pub(crate) fn select_rels<'store>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        main_cte_data: &Ciboulette2PostgresTable,
        inclusion_map: &BTreeMap<
            Vec<CibouletteResourceRelationshipDetails>,
            (
                Ciboulette2PostgresResponseType,
                Vec<CibouletteSortingElement>,
            ),
        >,
    ) -> Result<(), Ciboulette2SqlError> {
        for (included_list, (response_type, sort_fields_el)) in inclusion_map {
            let mut current_table = main_cte_data.clone();
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
                let relating_field = Ciboulette2PostgresRelatingField::new(
                    Ciboulette2PostgresTableField {
                        name: Ciboulette2PostgresSafeIdent::from(current_table.id().get_ident()),
                        alias: None,
                        cast: None,
                    },
                    current_table.clone(),
                    &current_rel_chain,
                    current_table.ciboulette_type().clone(),
                );
                let mut sort_additional_fields = Vec::with_capacity(sort_fields_el.len()); // FIXME Do better
                for sorting_element in sort_fields_el {
                    sort_additional_fields.push(Ciboulette2SqlAdditionalField::from_sorting_field(
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
					!matches!(include_el.relation_option(), CibouletteRelationshipOption::ManyToOne(x) if x.part_of_many_to_many().is_some()) && matches!(response_type, Ciboulette2PostgresResponseType::Object),
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
            }
        }
        Ok(())
    }
}
