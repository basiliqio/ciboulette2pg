use super::*;

impl<'request> Ciboulette2PgBuilder<'request> {
    /// Add the sorting key to the CTE table so that future table
    /// can reference them
    pub(crate) fn gen_sort_key_for_rel<'a>(
        state: &Ciboulette2PgBuilderState,
        main_cte_data: &Ciboulette2PgTable,
        rel_chain: &'a [CibouletteResourceRelationshipDetails],
    ) -> Result<Vec<Ciboulette2PgAdditionalField>, Ciboulette2PgError> {
        let mut sort_additional_fields = Vec::new();
        let additional_field_iter = match state.inclusion_map().get(rel_chain).map(|(_, x)| x) {
            Some(sort_fields_list) => {
                sort_additional_fields.reserve(sort_fields_list.len());
                for sorting_element in sort_fields_list {
                    sort_additional_fields.push(Ciboulette2PgAdditionalField::from_sorting_field(
                        sorting_element,
                        main_cte_data.ciboulette_type().clone(),
                    )?);
                }
                sort_additional_fields
            }
            None => sort_additional_fields,
        };
        Ok(additional_field_iter)
    }

    /// Wrapper for `gen_sort_key_for_rel` for the main table, which has an empty
    /// rel_chain
    pub(crate) fn gen_sort_key_for_main(
        state: &Ciboulette2PgBuilderState,
        main_cte_data: &Ciboulette2PgTable,
    ) -> Result<Vec<Ciboulette2PgAdditionalField>, Ciboulette2PgError> {
        Self::gen_sort_key_for_rel(state, main_cte_data, &[])
    }
}
