use super::*;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    /// Add additional fields to a final CTE select
    pub(super) fn handle_additionnal_params<'store, 'b, I>(
        &mut self,
        table: &Ciboulette2PostgresTable,
        additional_fields: I,
    ) -> Result<(), Ciboulette2SqlError>
    where
        I: Iterator<Item = &'b Ciboulette2SqlAdditionalField>,
    {
        for field in additional_fields {
            self.buf.write_all(b", ")?;
            self.insert_ident(&field.ident(), table)?;
            self.buf
                .write_all(format!(" AS \"{}\"", field.name()).as_bytes())?;
        }
        Ok(())
    }

    /// Get the relationships data for the main type
    pub(crate) fn get_relationships(
        ciboulette_store: &CibouletteStore,
        main_type: Arc<CibouletteResourceType>,
    ) -> Result<Ciboulette2SqlQueryRels<'request>, Ciboulette2SqlError> {
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
