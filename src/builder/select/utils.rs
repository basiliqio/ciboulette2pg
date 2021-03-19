use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(super) fn handle_additionnal_params<'b, I>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        table: &Ciboulette2PostgresTableSettings<'a>,
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

    pub(crate) fn gen_select_cte_single_rel(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        table: &Ciboulette2PostgresTableSettings<'a>,
        type_: &'a CibouletteResourceType<'a>,
        main_cte_table: &Ciboulette2PostgresTableSettings<'a>,
        field_id: &Ciboulette2PostgresSafeIdent<'a>,
        requirement_type: &CibouletteResponseRequiredType,
    ) -> Result<(), Ciboulette2SqlError> {
        self.gen_select_cte_final(
            &state,
            &table,
            &type_,
            [].iter(),
            matches!(requirement_type, CibouletteResponseRequiredType::Object),
        )?;
        self.buf.write_all(b" INNER JOIN ")?;
        self.write_table_info(&main_cte_table)?;
        self.buf.write_all(b" ON ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(table.id().get_ident(), None, None),
            &table,
        )?;
        self.buf.write_all(b" = ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(&field_id, None, None),
            &main_cte_table,
        )?;
        Ok(())
    }

    pub(crate) fn get_relationships(
        ciboulette_store: &'a CibouletteStore<'a>,
        main_type: &'a CibouletteResourceType<'a>,
    ) -> Result<Ciboulette2SqlQueryRels<'a>, Ciboulette2SqlError> {
        let main_single_relationships =
            crate::graph_walker::main::get_fields_single_rel(&ciboulette_store, &main_type)?;
        let rels: Vec<Ciboulette2PostgresRelationships> =
            crate::graph_walker::relationships::get_fields_multi_rels(
                &ciboulette_store,
                &main_type,
            )?;
        Ciboulette2SqlQueryRels::new(main_single_relationships, rels)
    }
}
