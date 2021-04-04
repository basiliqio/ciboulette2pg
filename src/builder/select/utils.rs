use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    /// Add additional fields to a final CTE select
    pub(super) fn handle_additionnal_params<'b, I>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        table: &Ciboulette2PostgresTable<'a>,
        additional_fields: I,
    ) -> Result<(), Ciboulette2SqlError>
    where
        'a: 'b,
        I: Iterator<Item = &'b Ciboulette2SqlAdditionalField<'a>>,
    {
        if !state.query().sorting().is_empty() {
            let id_as_additional = Ciboulette2SqlAdditionalField::try_from(table)?;
            self.buf.write_all(b", ")?;
            self.insert_ident(&id_as_additional.ident(), table)?;
            self.buf
                .write_all(format!(" AS \"{}\"", id_as_additional.name()).as_bytes())?;
        }
        {
            for field in additional_fields {
                self.buf.write_all(b", ")?;
                self.insert_ident(&field.ident(), table)?;
                self.buf
                    .write_all(format!(" AS \"{}\"", field.name()).as_bytes())?;
            }
        }
        Ok(())
    }

    /// Get the relationships data for the main type
    pub(crate) fn get_relationships(
        ciboulette_store: &'a CibouletteStore<'a>,
        main_type: Arc<CibouletteResourceType<'a>>,
    ) -> Result<Ciboulette2SqlQueryRels<'a>, Ciboulette2SqlError> {
        let main_single_relationships = crate::graph_walker::main::get_resource_single_rel(
            &ciboulette_store,
            main_type.clone(),
        )?;
        let rels: Vec<Ciboulette2PostgresMainResourceRelationships> =
            crate::graph_walker::relationships::get_resource_multi_rels(
                &ciboulette_store,
                main_type.clone(),
            )?;
        Ciboulette2SqlQueryRels::new(main_type, main_single_relationships, rels)
    }
}
