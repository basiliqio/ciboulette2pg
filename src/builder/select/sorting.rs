use super::*;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    pub(crate) fn gen_sort_key_for_main(
        state: &Ciboulette2PostgresBuilderState,
        main_cte_data: &Ciboulette2PostgresTable,
    ) -> Result<Vec<Ciboulette2SqlAdditionalField>, Ciboulette2SqlError> {
        let mut sort_additional_fields = Vec::new();
        let additional_field_iter = match state.inclusion_map().get(&vec![]).map(|(_, x)| x) {
            Some(sort_fields_list) => {
                sort_additional_fields.reserve(sort_fields_list.len());
                for sorting_element in sort_fields_list {
                    sort_additional_fields.push(Ciboulette2SqlAdditionalField::from_sorting_field(
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
}
