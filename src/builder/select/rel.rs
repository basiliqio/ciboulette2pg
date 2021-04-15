use super::*;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    /// Select into a new CTE a one-to-one relationship
    pub(crate) fn select_one_to_one_rels_routine<'store, F>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        main_type: Arc<CibouletteResourceType>,
        main_cte_data: &Ciboulette2PostgresTable,
        rels: &Ciboulette2SqlQueryRels<'request>,
        is_needed_cb: F,
    ) -> Result<(), Ciboulette2SqlError>
    where
        'store: 'request,
        F: Fn(
            &Ciboulette2PostgresBuilderState<'store, 'request>,
            &CibouletteResourceType,
        ) -> Option<Ciboulette2PostgresResponseType>,
    {
        for (rel_key, additional_fields) in rels
            .single_rels_keys()
            .iter()
            .zip(rels.single_rels_additional_fields().iter())
        {
            println!("Key : {} - {}", main_type.name(), rel_key);
            let rel_type: Arc<CibouletteResourceType> =
                main_type.get_relationship(&state.store(), rel_key)?;
            if let Some(requirement_type) = is_needed_cb(&state, &rel_type) {
                let rel_table: &Ciboulette2PostgresTable =
                    state.table_store().get(rel_type.name().as_str())?;
                let rel_table_cte: Ciboulette2PostgresTable =
                    rel_table.to_cte(CIBOULETTE_EMPTY_IDENT, CIBOULETTE_DATA_SUFFIX)?;
                self.buf.write_all(b", ")?;
                self.write_table_info(&rel_table_cte)?;
                self.buf.write_all(b" AS (")?;
                self.gen_select_cte_single_rel(
                    &state,
                    &rel_table,
                    rel_type.clone(),
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
        rels: &CibouletteRelationshipManyToManyOption
    ) -> Result<[Ciboulette2SqlAdditionalField; 2], Ciboulette2SqlError> {
        Ok([
            Ciboulette2SqlAdditionalField::new(
                Ciboulette2PostgresTableField::new(
                    Ciboulette2PostgresSafeIdent::try_from(rels.keys()[0].1.clone())?,
                    None,
                    None,
                ),
                Ciboulette2SqlAdditionalFieldType::Relationship,
                rels.keys()[0].0.clone(),
            )?,
            Ciboulette2SqlAdditionalField::new(
                Ciboulette2PostgresTableField::new(
                    Ciboulette2PostgresSafeIdent::try_from(rels.keys()[1].1.clone())?,
                    None,
                    None,
                ),
                Ciboulette2SqlAdditionalFieldType::Relationship,
                rels.keys()[1].0.clone(),
            )?,
        ])
    }

    /// Create 2 additional fields to select containing the linking key of the related table in the bucket table
    fn gen_additional_params_one_to_many_rels(
        rels: &CibouletteRelationshipOneToManyOption
    ) -> Result<[Ciboulette2SqlAdditionalField; 1], Ciboulette2SqlError> {
        Ok([Ciboulette2SqlAdditionalField::new(
            Ciboulette2PostgresTableField::new(
                Ciboulette2PostgresSafeIdent::try_from(rels.many_table_key())?,
                None,
                None,
            ),
            Ciboulette2SqlAdditionalFieldType::Relationship,
            rels.many_table().clone(),
        )?])
    }

    /// Create new CTE with relationships data and relationships linking data
    pub(crate) fn select_multi_rels_routine<'store, F>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        main_cte_data: &Ciboulette2PostgresTable,
        rels: &[Ciboulette2PostgresMainResourceRelationships<'request>],
        is_needed_cb: F,
    ) -> Result<(), Ciboulette2SqlError>
    where
        'store: 'request,
        F: Fn(
            &Ciboulette2PostgresBuilderState<'store, 'request>,
            &CibouletteResourceType,
        ) -> Option<Ciboulette2PostgresResponseType>,
    {
        let rel_iter = rels.iter().peekable();
        for Ciboulette2PostgresMainResourceRelationships {
            type_: rel_type,
            rel_opt: bucket,
            values: _rel_ids,
        } in rel_iter
        {
            if let Some(rel_requirement_type) = is_needed_cb(&state, &rel_type) {
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
                            additional_params.iter(),
                            main_cte_data,
                        )?;
                        self.buf.write_all(b"), ")?;
                        let (left_additional_param, right_additional_param) =
                            match additional_params[0].ciboulette_type().as_ref() == main_cte_data.ciboulette_type().as_ref() // Match the type that's not compatible with the main one
						{
							true => (&additional_params[1], &additional_params[0]),
							false => (&additional_params[0], &additional_params[1]),
						};
                        self.gen_select_many_to_many_rels_rel_data(
                            &rel_cte_data,
                            state,
                            rel_table,
                            &rel_cte_rel_data,
                            &left_additional_param,
                            &right_additional_param,
                            &rel_requirement_type,
                        )?;
                        self.add_working_table(&rel_table, (rel_cte_data, rel_requirement_type));
                    }
                    Ciboulette2PostgresMultiRelationships::ManyToOne(opt)
                        if opt.part_of_many_to_many().is_none() =>
                    {
                        self.buf.write_all(b", ")?;
                        let additional_params = Self::gen_additional_params_one_to_many_rels(opt)?;
                        let rel_table = state.table_store().get(rel_type.name().as_str())?;
                        let rel_cte_data =
                            rel_table.to_cte(CIBOULETTE_REL_PREFIX, CIBOULETTE_DATA_SUFFIX)?;
                        self.write_table_info(&rel_cte_data)?;
                        self.buf.write_all(b" AS (")?;
                        self.gen_select_one_to_many_rels_data(
                            state,
                            &rel_table,
                            &main_cte_data,
                            additional_params.iter(),
                            &opt,
                            &rel_requirement_type,
                        )?;
                        self.add_working_table(&rel_table, (rel_cte_data, rel_requirement_type));
                    }
                    _ => continue,
                }
            }
        }
        Ok(())
    }

    /// Generate the CTE to include a relationship (many-to-many) linking data to the query
    fn gen_select_one_to_many_rels_data<'store, 'b, I>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        rel_table: &Ciboulette2PostgresTable,
        main_cte_data: &Ciboulette2PostgresTable,
        additional_params: I,
        opt: &CibouletteRelationshipOneToManyOption,
        rel_requirement_type: &Ciboulette2PostgresResponseType,
    ) -> Result<(), Ciboulette2SqlError>
    where
        'store: 'request + 'b,
        I: Iterator<Item = &'b Ciboulette2SqlAdditionalField>,
    {
        let many_table_key_field = Ciboulette2PostgresSafeIdent::try_from(opt.many_table_key())?;
        let many_table_key = Ciboulette2PostgresTableField::new(many_table_key_field, None, None);
        let relating_field = Ciboulette2PostgresRelatingField::new(
            many_table_key.clone(),
            rel_table.clone(),
            main_cte_data.ciboulette_type().clone(),
        );
        self.gen_select_cte_final(
            &state,
            &rel_table,
            rel_table.ciboulette_type().clone(),
            Some(relating_field),
            additional_params,
            matches!(
                rel_requirement_type,
                Ciboulette2PostgresResponseType::Object
            ),
        )?;
        self.buf.write_all(b" WHERE ")?;
        self.insert_ident(&many_table_key, rel_table)?;
        self.buf.write_all(b" = ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new(main_cte_data.id().get_ident().clone(), None, None),
            main_cte_data,
        )?;
        self.buf.write_all(b")")?;
        Ok(())
    }

    /// Generate the CTE to include a relationship (many-to-many) linking data to the query
    #[allow(clippy::too_many_arguments)] //FIXME
    fn gen_select_many_to_many_rels_rel_data<'store>(
        &mut self,
        rel_cte_data: &Ciboulette2PostgresTable,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        rel_table: &Ciboulette2PostgresTable,
        rel_cte_rel_data: &Ciboulette2PostgresTable,
        left_additional_params: &Ciboulette2SqlAdditionalField,
        right_additional_params: &Ciboulette2SqlAdditionalField,
        rel_requirement_type: &Ciboulette2PostgresResponseType,
    ) -> Result<(), Ciboulette2SqlError>
    where
        'store: 'request,
    {
        let many_table_key =
            Ciboulette2PostgresTableField::new(right_additional_params.name().clone(), None, None);
        let relating_field = Ciboulette2PostgresRelatingField::new(
            many_table_key,
            rel_cte_rel_data.clone(),
            state.main_type().clone(),
        );
        self.write_table_info(rel_cte_data)?;
        self.buf.write_all(b" AS (")?;
        self.gen_select_cte_final(
            &state,
            &rel_table,
            rel_table.ciboulette_type().clone(),
            Some(relating_field),
            [].iter(),
            matches!(
                rel_requirement_type,
                Ciboulette2PostgresResponseType::Object
            ),
        )?;
        self.buf.write_all(b" INNER JOIN ")?;
        self.write_table_info(rel_cte_rel_data)?;
        self.buf.write_all(b" ON ")?;
        self.compare_fields(
            rel_cte_rel_data,
            &Ciboulette2PostgresTableField::new(left_additional_params.name().clone(), None, None),
            &rel_table,
            &Ciboulette2PostgresTableField::new(rel_table.id().get_ident().clone(), None, None),
        )?;
        self.buf.write_all(b")")?;
        Ok(())
    }

    /// Generate the CTE to include a relationship (many-to-many) data to the query
    fn gen_select_many_to_many_rels_data<'store, 'b, I>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        rel_rel_table: &Ciboulette2PostgresTable,
        bucket: &Ciboulette2PostgresMultiRelationships,
        additional_params: I,
        main_cte_data: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError>
    where
        I: Iterator<Item = &'b Ciboulette2SqlAdditionalField>,
        'store: 'request,
    {
        let dest_resource = state
            .store()
            .get_type(bucket.dest_resource().name().as_str())
            .unwrap(); //FIXME
        self.buf.write_all(b"SELECT ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new(
                rel_rel_table.id().get_ident().clone(),
                Some(CIBOULETTE_ID_IDENT),
                Some(TEXT_IDENT),
            ),
            rel_rel_table,
        )?;
        self.handle_additionnal_params(&state, &rel_rel_table, additional_params)?;
        self.gen_sorting_keys(&rel_rel_table, dest_resource.clone(), &state.query())?;
        self.buf.write_all(b" FROM ")?;
        self.write_table_info(rel_rel_table)?;
        self.buf.write_all(b" INNER JOIN ")?;
        self.write_table_info(&main_cte_data)?;
        self.buf.write_all(b" ON ")?;
        self.compare_fields(
            &rel_rel_table,
            &Ciboulette2PostgresTableField::new(
                Ciboulette2PostgresSafeIdent::try_from(bucket.dest_key(state.main_type())?)?,
                None,
                None,
            ),
            &main_cte_data,
            &Ciboulette2PostgresTableField::new(main_cte_data.id().get_ident().clone(), None, None),
        )?;
        Ok(())
    }

    /// Generate a one-to-one relationship
    pub(crate) fn gen_select_cte_single_rel<'store>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        table: &'store Ciboulette2PostgresTable,
        type_: Arc<CibouletteResourceType>,
        main_cte_table: &Ciboulette2PostgresTable,
        field_id: &Ciboulette2PostgresSafeIdent,
        requirement_type: &Ciboulette2PostgresResponseType,
    ) -> Result<(), Ciboulette2SqlError> {
        let table_field =
            Ciboulette2PostgresTableField::new(table.id().get_ident().clone(), None, None);
        let relating_field = Ciboulette2PostgresRelatingField::new(
            table_field.clone(),
            main_cte_table.clone(),
            main_cte_table.ciboulette_type().clone(),
        );
        self.gen_select_cte_final(
            &state,
            &table,
            type_,
            Some(relating_field),
            [].iter(),
            matches!(requirement_type, Ciboulette2PostgresResponseType::Object),
        )?;
        self.buf.write_all(b" INNER JOIN ")?;
        self.write_table_info(&main_cte_table)?;
        self.buf.write_all(b" ON ")?;
        self.insert_ident(&table_field, &table)?;
        self.buf.write_all(b" = ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new(field_id.clone(), None, None),
            &main_cte_table,
        )?;
        Ok(())
    }

    /// Generate the table that'll be used to in the query to select one-to-many relationships
    fn gen_rel_select_tables(
        rel_rel_table: &Ciboulette2PostgresTable,
        rel_table: &Ciboulette2PostgresTable,
    ) -> Result<(Ciboulette2PostgresTable, Ciboulette2PostgresTable), Ciboulette2SqlError> {
        let rel_cte_rel_data =
            rel_rel_table.to_cte(CIBOULETTE_REL_PREFIX, CIBOULETTE_REL_DATA_SUFFIX)?;
        let rel_cte_data = rel_table.to_cte(CIBOULETTE_REL_PREFIX, CIBOULETTE_DATA_SUFFIX)?;
        Ok((rel_cte_rel_data, rel_cte_data))
    }
}
