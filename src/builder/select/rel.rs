use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    /// Select into a new CTE a one-to-one relationship
    pub(crate) fn select_one_to_one_rels_routine(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        main_cte_data: &Ciboulette2PostgresTable<'a>,
        rels: &Ciboulette2SqlQueryRels<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        for (rel_key, additional_fields) in rels
            .single_rels_keys()
            .iter()
            .zip(rels.single_rels_additional_fields().iter())
        {
            let rel_type: &CibouletteResourceType = state
                .main_type()
                .get_relationship(&state.store(), rel_key)?;
            if let Some(requirement_type) = state.is_type_needed(&rel_type) {
                let rel_table: &Ciboulette2PostgresTable = state.table_store().get(rel_key)?;
                let rel_table_cte: Ciboulette2PostgresTable =
                    rel_table.to_cte(Cow::Owned(format!("cte_{}_data", rel_table.name())))?;
                self.buf.write_all(b", ")?;
                self.write_table_info(&rel_table_cte)?;
                self.buf.write_all(b" AS (")?;
                self.gen_select_cte_single_rel(
                    &state,
                    &rel_table,
                    &rel_type,
                    &main_cte_data,
                    &additional_fields.name(),
                    &requirement_type,
                )?;
                self.buf.write_all(b")")?;
                self.add_working_table(&rel_table, (rel_table_cte, requirement_type));
            }
        }
        Ok(())
    }

    /// Create 2 additional fields to select containing the linking key of the related table in the bucket table
    fn gen_additional_params_many_to_many_rels(
        rels: &'a CibouletteRelationshipManyToManyOption
    ) -> Result<[Ciboulette2SqlAdditionalField<'a>; 2], Ciboulette2SqlError> {
        Ok([
            Ciboulette2SqlAdditionalField::new(
                Ciboulette2PostgresTableField::new_owned(
                    Ciboulette2PostgresSafeIdent::try_from(rels.keys()[0].1.as_str())?,
                    None,
                    None,
                ),
                Ciboulette2SqlAdditionalFieldType::Relationship,
            )?,
            Ciboulette2SqlAdditionalField::new(
                Ciboulette2PostgresTableField::new_owned(
                    Ciboulette2PostgresSafeIdent::try_from(rels.keys()[1].1.as_str())?,
                    None,
                    None,
                ),
                Ciboulette2SqlAdditionalFieldType::Relationship,
            )?,
        ])
    }

    /// Create 2 additional fields to select containing the linking key of the related table in the bucket table
    fn gen_additional_params_one_to_many_rels(
        rels: &'a CibouletteRelationshipOneToManyOption
    ) -> Result<[Ciboulette2SqlAdditionalField<'a>; 1], Ciboulette2SqlError> {
        Ok([Ciboulette2SqlAdditionalField::new(
            Ciboulette2PostgresTableField::new_owned(
                Ciboulette2PostgresSafeIdent::try_from(rels.many_table_key().as_str())?,
                None,
                None,
            ),
            Ciboulette2SqlAdditionalFieldType::Relationship,
        )?])
    }

    /// Create new CTE with relationships data and relationships linking data
    pub(crate) fn select_multi_rels_routine(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        main_cte_data: &Ciboulette2PostgresTable<'a>,
        rels: &[Ciboulette2PostgresMainResourceRelationships<'a>],
    ) -> Result<(), Ciboulette2SqlError> {
        let rel_iter = rels.iter().peekable();
        for Ciboulette2PostgresMainResourceRelationships {
            type_: rel_type,
            rel_opt: bucket,
            values: _rel_ids,
        } in rel_iter
        {
            if let Some(rel_requirement_type) = state.is_type_needed(&rel_type) {
                match bucket {
                    Ciboulette2PostgresMultiRelationships::ManyToMany(opt) => {
                        self.buf.write_all(b", ")?;
                        let additional_params = Self::gen_additional_params_many_to_many_rels(opt)?;
                        let rel_table = state.table_store().get(rel_type.name().as_str())?;
                        let rel_rel_table = state
                            .table_store()
                            .get(bucket.dest_resource().name().as_str())?;
                        let (rel_cte_rel_data, rel_cte_data) =
                            Self::gen_rel_select_tables(rel_rel_table, rel_table)?;
                        self.write_table_info(&rel_cte_rel_data)?;
                        self.buf.write_all(b" AS (")?;
                        self.gen_select_many_to_many_rels_data(
                            state,
                            rel_rel_table,
                            bucket,
                            &additional_params,
                            main_cte_data,
                            &rel_requirement_type,
                        )?;
                        self.buf.write_all(b"), ")?;
                        self.gen_select_many_to_many_rels_rel_data(
                            &rel_cte_data,
                            state,
                            rel_table,
                            &rel_cte_rel_data,
                            &additional_params[0],
                            &rel_requirement_type,
                        )?;
                        self.add_working_table(&rel_table, (rel_cte_data, rel_requirement_type));
                        self.add_working_table(
                            &rel_rel_table,
                            (rel_cte_rel_data, rel_requirement_type),
                        );
                    }
                    Ciboulette2PostgresMultiRelationships::OneToMany(opt)
                        if opt.part_of_many_to_many().is_some() =>
                    {
                        continue
                    }
                    Ciboulette2PostgresMultiRelationships::OneToMany(opt) => {
                        self.buf.write_all(b", ")?;
                        let additional_params = Self::gen_additional_params_one_to_many_rels(opt)?;
                        let rel_table = state.table_store().get(rel_type.name().as_str())?;
                        let rel_cte_data = rel_table
                            .to_cte(Cow::Owned(format!("cte_rel_{}_data", rel_table.name())))?;
                        self.write_table_info(&rel_cte_data)?;
                        self.buf.write_all(b" AS (")?;
                        self.gen_select_one_to_many_rels_data(
                            state,
                            &rel_table,
                            &main_cte_data,
                            &additional_params,
                            &opt,
                            &rel_requirement_type,
                        )?;
                        self.add_working_table(&rel_table, (rel_cte_data, rel_requirement_type));
                    }
                }
            }
        }
        Ok(())
    }

    /// Generate the CTE to include a relationship (many-to-many) linking data to the query
    fn gen_select_one_to_many_rels_data(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        rel_table: &Ciboulette2PostgresTable<'a>,
        main_cte_data: &Ciboulette2PostgresTable<'a>,
        additional_params: &[Ciboulette2SqlAdditionalField<'a>],
        opt: &CibouletteRelationshipOneToManyOption<'a>,
        rel_requirement_type: &CibouletteResponseRequiredType,
    ) -> Result<(), Ciboulette2SqlError> {
        self.gen_select_cte_final(
            &state,
            &rel_table,
            &rel_table.ciboulette_type(),
            additional_params.iter(),
            matches!(rel_requirement_type, CibouletteResponseRequiredType::Object),
        )?;
        self.buf.write_all(b" WHERE ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(
                &Ciboulette2PostgresSafeIdent::try_from(opt.many_table_key().as_str())?,
                None,
                None,
            ),
            rel_table,
        )?;
        self.buf.write_all(b" = ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(main_cte_data.id().get_ident(), None, None),
            main_cte_data,
        )?;
        self.buf.write_all(b")")?;
        Ok(())
    }

    /// Generate the CTE to include a relationship (many-to-many) linking data to the query
    fn gen_select_many_to_many_rels_rel_data(
        &mut self,
        rel_cte_data: &Ciboulette2PostgresTable<'a>,
        state: &Ciboulette2PostgresBuilderState<'a>,
        rel_table: &Ciboulette2PostgresTable<'a>,
        rel_cte_rel_data: &Ciboulette2PostgresTable<'a>,
        left_additional_params: &Ciboulette2SqlAdditionalField<'a>,
        rel_requirement_type: &CibouletteResponseRequiredType,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(rel_cte_data)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte_final(
            &state,
            &rel_table,
            &rel_table.ciboulette_type(),
            [].iter(),
            matches!(rel_requirement_type, CibouletteResponseRequiredType::Object),
        )?;
        self.buf.write_all(b" INNER JOIN ")?;
        self.write_table_info(rel_cte_rel_data)?;
        self.buf.write_all(b" ON ")?;
        self.compare_fields(
            rel_cte_rel_data,
            &Ciboulette2PostgresTableField::new_ref(left_additional_params.name(), None, None),
            &rel_table,
            &Ciboulette2PostgresTableField::new_ref(rel_table.id().get_ident(), None, None),
        )?;
        self.buf.write_all(b")")?;
        Ok(())
    }

    /// Generate the CTE to include a relationship (many-to-many) data to the query
    fn gen_select_many_to_many_rels_data(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        rel_rel_table: &Ciboulette2PostgresTable<'a>,
        bucket: &Ciboulette2PostgresMultiRelationships<'a>,
        additional_params: &[Ciboulette2SqlAdditionalField<'a>],
        main_cte_data: &Ciboulette2PostgresTable<'a>,
        rel_requirement_type: &CibouletteResponseRequiredType,
    ) -> Result<(), Ciboulette2SqlError> {
        let dest_resource = state
            .store()
            .get_type(bucket.dest_resource().name().as_str())
            .unwrap(); //FIXME
        self.gen_select_cte_final(
            &state,
            &rel_rel_table,
            &dest_resource,
            additional_params.iter(),
            matches!(rel_requirement_type, CibouletteResponseRequiredType::Object),
        )?;
        self.buf.write_all(b" INNER JOIN ")?;
        self.write_table_info(&main_cte_data)?;
        self.buf.write_all(b" ON ")?;
        self.compare_fields(
            &rel_rel_table,
            &Ciboulette2PostgresTableField::new_owned(
                Ciboulette2PostgresSafeIdent::try_from(bucket.dest_key(state.main_type())?)?,
                None,
                None,
            ),
            &main_cte_data,
            &Ciboulette2PostgresTableField::new_ref(main_cte_data.id().get_ident(), None, None),
        )?;
        Ok(())
    }

    /// Generate a one-to-one relationship
    pub(crate) fn gen_select_cte_single_rel(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        table: &Ciboulette2PostgresTable<'a>,
        type_: &'a CibouletteResourceType<'a>,
        main_cte_table: &Ciboulette2PostgresTable<'a>,
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

    /// Generate the table that'll be used to in the query to select one-to-many relationships
    fn gen_rel_select_tables(
        rel_rel_table: &'a Ciboulette2PostgresTable<'a>,
        rel_table: &'a Ciboulette2PostgresTable<'a>,
    ) -> Result<(Ciboulette2PostgresTable<'a>, Ciboulette2PostgresTable<'a>), Ciboulette2SqlError>
    {
        let rel_cte_rel_data =
            rel_rel_table.to_cte(Cow::Owned(format!("cte_rel_{}_rel_data", rel_table.name())))?;
        let rel_cte_data =
            rel_table.to_cte(Cow::Owned(format!("cte_rel_{}_data", rel_table.name())))?;
        Ok((rel_cte_rel_data, rel_cte_data))
    }
}
