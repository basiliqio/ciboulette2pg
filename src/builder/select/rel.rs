use super::*;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    // /// Select into a new CTE a one-to-one relationship
    // pub(crate) fn select_one_to_one_rels_routine<'store, F>(
    //     &mut self,
    //     state: &Ciboulette2PostgresBuilderState<'store, 'request>,
    //     main_cte_data: &Ciboulette2PostgresTable,
    //     single_rels: &[Ciboulette2PostgresResourceSingleRelationships],
    //     single_rels_additional_fields: &[Ciboulette2SqlAdditionalField],
    //     is_needed_cb: F,
    // ) -> Result<(), Ciboulette2SqlError>
    // where
    //     'store: 'request,
    //     F: Fn(
    //         &Ciboulette2PostgresBuilderState<'store, 'request>,
    //         &CibouletteResourceType,
    //     ) -> Option<Ciboulette2PostgresResponseType>,
    // {
    //     for (rel_key, additional_fields) in
    //         single_rels.iter().zip(single_rels_additional_fields.iter())
    //     {
    //         if let Some(requirement_type) = is_needed_cb(&state, &rel_key.type_()) {
    //             let rel_table: &Ciboulette2PostgresTable =
    //                 state.table_store().get(rel_key.type_().name().as_str())?;
    //             let rel_table_cte: Ciboulette2PostgresTable =
    //                 rel_table.to_cte(&mut *self, CIBOULETTE_REL_PREFIX, CIBOULETTE_DATA_SUFFIX)?;
    //             self.buf.write_all(b", ")?;
    //             self.write_table_info(&rel_table_cte)?;
    //             self.buf.write_all(b" AS (")?;
    //             self.gen_select_cte_single_rel(
    //                 &state,
    //                 &rel_table,
    //                 &main_cte_data,
    //                 &additional_fields.name(),
    //                 rel_key.rel_details(),
    //                 &requirement_type,
    //             )?;
    //             self.buf.write_all(b")")?;
    //             self.add_working_table(rel_table_cte, requirement_type);
    //         }
    //     }
    //     Ok(())
    // }

    // /// Create 2 additional fields to select containing the linking key of the related table in the bucket table
    // fn gen_additional_params_many_to_many_rels(
    //     rels: &CibouletteRelationshipManyToManyOption
    // ) -> Result<[Ciboulette2SqlAdditionalField; 2], Ciboulette2SqlError> {
    //     Ok([
    //         Ciboulette2SqlAdditionalField::new(
    //             Ciboulette2PostgresTableField::new(
    //                 Ciboulette2PostgresSafeIdent::try_from(rels.keys()[0].1.clone())?,
    //                 None,
    //                 None,
    //             ),
    //             Ciboulette2SqlAdditionalFieldType::Relationship,
    //             rels.keys()[0].0.clone(),
    //         ),
    //         Ciboulette2SqlAdditionalField::new(
    //             Ciboulette2PostgresTableField::new(
    //                 Ciboulette2PostgresSafeIdent::try_from(rels.keys()[1].1.clone())?,
    //                 None,
    //                 None,
    //             ),
    //             Ciboulette2SqlAdditionalFieldType::Relationship,
    //             rels.keys()[1].0.clone(),
    //         ),
    //     ])
    // }

    // /// Create 2 additional fields to select containing the linking key of the related table in the bucket table
    // fn gen_additional_params_one_to_many_rels(
    //     rels: &CibouletteRelationshipOneToManyOption
    // ) -> Result<[Ciboulette2SqlAdditionalField; 1], Ciboulette2SqlError> {
    //     Ok([Ciboulette2SqlAdditionalField::new(
    //         Ciboulette2PostgresTableField::new(
    //             Ciboulette2PostgresSafeIdent::try_from(rels.many_table_key())?,
    //             None,
    //             None,
    //         ),
    //         Ciboulette2SqlAdditionalFieldType::Relationship,
    //         rels.many_table().clone(),
    //     )])
    // }

    // /// Create new CTE with relationships data and relationships linking data
    // pub(crate) fn select_multi_rels_routine<'store, F>(
    //     &mut self,
    //     state: &Ciboulette2PostgresBuilderState<'store, 'request>,
    //     main_cte_data: &Ciboulette2PostgresTable,
    //     rels: &BTreeMap<ArcStr, Ciboulette2PostgresMultiRelationships<'request>>,
    //     is_needed_cb: F,
    // ) -> Result<(), Ciboulette2SqlError>
    // where
    //     'store: 'request,
    //     F: Fn(
    //         &Ciboulette2PostgresBuilderState<'store, 'request>,
    //         &CibouletteResourceType,
    //     ) -> Option<Ciboulette2PostgresResponseType>,
    // {
    //     let rel_iter = rels.values().peekable();
    //     for multi_rel in rel_iter {
    //         if let Some(rel_requirement_type) = is_needed_cb(&state, &multi_rel.type_()) {
    //             match multi_rel.rel_opt() {
    //                 Ciboulette2PostgresMultiRelationshipsType::ManyToMany(opt) => {
    //                     self.buf.write_all(b", ")?;
    //                     let additional_params = Self::gen_additional_params_many_to_many_rels(opt)?;
    //                     let rel_table =
    //                         state.table_store().get(multi_rel.type_().name().as_str())?;
    //                     let rel_rel_table = state
    //                         .table_store()
    //                         .get(multi_rel.rel_opt().dest_resource().name().as_str())?;
    //                     let (rel_cte_rel_data, rel_cte_data) =
    //                         self.gen_rel_select_tables(rel_rel_table, rel_table)?;
    //                     self.write_table_info(&rel_cte_rel_data)?;
    //                     self.buf.write_all(b" AS (")?;
    //                     self.gen_select_many_to_many_rels_data(
    //                         state,
    //                         rel_rel_table,
    //                         &multi_rel,
    //                         additional_params.iter(),
    //                         main_cte_data,
    //                     )?;
    //                     self.buf.write_all(b"), ")?;
    //                     let (left_additional_param, right_additional_param) =
    //                         match additional_params[0].ciboulette_type().as_ref() == main_cte_data.ciboulette_type().as_ref() // Match the type that's not compatible with the main one
    // 					{
    // 						true => (&additional_params[1], &additional_params[0]),
    // 						false => (&additional_params[0], &additional_params[1]),
    // 					};
    //                     self.gen_select_many_to_many_rels_rel_data(
    //                         &rel_cte_data,
    //                         state,
    //                         rel_table,
    //                         &rel_cte_rel_data,
    //                         &left_additional_param,
    //                         &right_additional_param,
    //                         &rel_requirement_type,
    //                         multi_rel.rel_details(),
    //                     )?;
    //                     self.add_working_table(rel_cte_data, rel_requirement_type);
    //                 }
    //                 Ciboulette2PostgresMultiRelationshipsType::OneToMany(opt)
    //                     if opt.part_of_many_to_many().is_none() =>
    //                 {
    //                     self.buf.write_all(b", ")?;
    //                     let additional_params = Self::gen_additional_params_one_to_many_rels(opt)?;
    //                     let rel_table =
    //                         state.table_store().get(multi_rel.type_().name().as_str())?;
    //                     let rel_cte_data = rel_table.to_cte(
    //                         &mut *self,
    //                         CIBOULETTE_REL_PREFIX,
    //                         CIBOULETTE_DATA_SUFFIX,
    //                     )?;
    //                     self.write_table_info(&rel_cte_data)?;
    //                     self.buf.write_all(b" AS (")?;
    //                     self.gen_select_one_to_many_rels_data(
    //                         state,
    //                         &rel_table,
    //                         &main_cte_data,
    //                         additional_params.iter(),
    //                         &opt,
    //                         multi_rel.rel_details(),
    //                         &rel_requirement_type,
    //                     )?;
    //                     self.add_working_table(rel_cte_data, rel_requirement_type);
    //                 }
    //                 _ => {
    //                     continue;
    //                 }
    //             }
    //         }
    //     }
    //     Ok(())
    // }

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
                self.gen_select_cte(
					state,
					&next_table,
					working_table.ciboulette_type().clone(),
					Some(relating_field),
					sort_additional_fields.iter(),
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

    // /// Generate the CTE to include a relationship (many-to-many) linking data to the query
    // #[allow(clippy::too_many_arguments)] //FIXME
    // fn gen_select_one_to_many_rels_data<'store, 'b, I>(
    //     &mut self,
    //     state: &Ciboulette2PostgresBuilderState<'store, 'request>,
    //     rel_table: &Ciboulette2PostgresTable,
    //     main_cte_data: &Ciboulette2PostgresTable,
    //     additional_params: I,
    //     opt: &CibouletteRelationshipOneToManyOption,
    //     rel_details: &CibouletteResourceRelationshipDetails,
    //     rel_requirement_type: &Ciboulette2PostgresResponseType,
    // ) -> Result<(), Ciboulette2SqlError>
    // where
    //     'store: 'request + 'b,
    //     I: Iterator<Item = &'b Ciboulette2SqlAdditionalField>,
    // {
    //     let many_table_key_field = Ciboulette2PostgresSafeIdent::try_from(opt.many_table_key())?;
    //     let many_table_key = Ciboulette2PostgresTableField::new(many_table_key_field, None, None);
    //     let relating_field = Ciboulette2PostgresRelatingField::new(
    //         many_table_key.clone(),
    //         rel_table.clone(),
    //         rel_details.relation_alias().clone(),
    //         main_cte_data.ciboulette_type().clone(),
    //     );
    //     self.gen_select_cte_final(
    //         &state,
    //         &rel_table,
    //         rel_table.ciboulette_type().clone(),
    //         Some(relating_field),
    //         additional_params,
    //         matches!(
    //             rel_requirement_type,
    //             Ciboulette2PostgresResponseType::Object
    //         ),
    //     )?;
    //     Self::gen_inner_join(&mut self.buf, state, &main_cte_data, &rel_details, None)?;
    //     self.buf.write_all(b" WHERE ")?;
    //     self.insert_ident(&many_table_key, rel_table)?;
    //     self.buf.write_all(b" = ")?;
    //     self.insert_ident(
    //         &Ciboulette2PostgresTableField::new(CIBOULETTE_MAIN_IDENTIFIER, None, None),
    //         main_cte_data,
    //     )?;
    //     self.buf.write_all(b")")?;
    //     Ok(())
    // }

    // /// Generate the CTE to include a relationship (many-to-many) linking data to the query
    // #[allow(clippy::too_many_arguments)] //FIXME
    // fn gen_select_many_to_many_rels_rel_data<'store>(
    //     &mut self,
    //     rel_cte_data: &Ciboulette2PostgresTable,
    //     state: &Ciboulette2PostgresBuilderState<'store, 'request>,
    //     rel_table: &Ciboulette2PostgresTable,
    //     rel_cte_rel_data: &Ciboulette2PostgresTable,
    //     left_additional_params: &Ciboulette2SqlAdditionalField,
    //     right_additional_params: &Ciboulette2SqlAdditionalField,
    //     rel_requirement_type: &Ciboulette2PostgresResponseType,
    //     rel_details: &CibouletteResourceRelationshipDetails,
    // ) -> Result<(), Ciboulette2SqlError>
    // where
    //     'store: 'request,
    // {
    //     let many_table_key =
    //         Ciboulette2PostgresTableField::new(right_additional_params.name().clone(), None, None);
    //     let relating_field = Ciboulette2PostgresRelatingField::new(
    //         many_table_key,
    //         rel_cte_rel_data.clone(),
    //         rel_details.relation_alias().clone(),
    //         state.main_type().clone(),
    //     );
    //     self.write_table_info(rel_cte_data)?;
    //     self.buf.write_all(b" AS (")?;
    //     self.gen_select_cte_final(
    //         &state,
    //         &rel_table,
    //         rel_table.ciboulette_type().clone(),
    //         Some(relating_field),
    //         [].iter(),
    //         matches!(
    //             rel_requirement_type,
    //             Ciboulette2PostgresResponseType::Object
    //         ),
    //     )?;
    //     self.buf.write_all(b" INNER JOIN ")?;
    //     self.write_table_info(rel_cte_rel_data)?;
    //     self.buf.write_all(b" ON ")?;
    //     self.compare_fields(
    //         rel_cte_rel_data,
    //         &Ciboulette2PostgresTableField::new(left_additional_params.name().clone(), None, None),
    //         &rel_table,
    //         &Ciboulette2PostgresTableField::new(rel_table.id().get_ident().clone(), None, None),
    //     )?;
    //     self.buf.write_all(b")")?;
    //     Ok(())
    // }

    // /// Generate the CTE to include a relationship (many-to-many) data to the query
    // fn gen_select_many_to_many_rels_data<'store, 'b, I>(
    //     &mut self,
    //     state: &Ciboulette2PostgresBuilderState<'store, 'request>,
    //     rel_rel_table: &Ciboulette2PostgresTable,
    //     bucket: &Ciboulette2PostgresMultiRelationships,
    //     additional_params: I,
    //     main_cte_data: &Ciboulette2PostgresTable,
    // ) -> Result<(), Ciboulette2SqlError>
    // where
    //     I: Iterator<Item = &'b Ciboulette2SqlAdditionalField>,
    //     'store: 'request,
    // {
    //     let dest_resource = state
    //         .store()
    //         .get_type(bucket.rel_opt().dest_resource().name().as_str())?;
    //     self.buf.write_all(b"SELECT ")?;
    //     self.insert_ident(
    //         &Ciboulette2PostgresTableField::new(
    //             rel_rel_table.id().get_ident().clone(),
    //             Some(CIBOULETTE_ID_IDENT),
    //             Some(TEXT_IDENT),
    //         ),
    //         rel_rel_table,
    //     )?;
    //     self.handle_additionnal_params(&rel_rel_table, additional_params)?;
    //     self.buf.write_all(b" FROM ")?;
    //     self.write_table_info(rel_rel_table)?;
    //     // Self::gen_inner_join(&mut self.buf, state, &main_cte_data, &rel_details, Some(rel_rel_table))?;
    //     self.buf.write_all(b" INNER JOIN ")?;
    //     self.write_table_info(&main_cte_data)?;
    //     self.buf.write_all(b" ON ")?;
    //     self.compare_fields(
    //         &rel_rel_table,
    //         &Ciboulette2PostgresTableField::new(
    //             Ciboulette2PostgresSafeIdent::try_from(
    //                 bucket.rel_opt().dest_key(state.main_type())?,
    //             )?,
    //             None,
    //             None,
    //         ),
    //         &main_cte_data,
    //         &Ciboulette2PostgresTableField::new(CIBOULETTE_MAIN_IDENTIFIER, None, None),
    //     )?;
    //     Ok(())
    // }

    // /// Generate a one-to-one relationship
    // pub(crate) fn gen_select_cte_single_rel<'store>(
    //     &mut self,
    //     state: &Ciboulette2PostgresBuilderState<'store, 'request>,
    //     table: &'store Ciboulette2PostgresTable,
    //     main_cte_table: &Ciboulette2PostgresTable,
    //     field_id: &Ciboulette2PostgresSafeIdent,
    //     rel_details: &CibouletteResourceRelationshipDetails,
    //     requirement_type: &Ciboulette2PostgresResponseType,
    // ) -> Result<(), Ciboulette2SqlError> {
    //     let table_field =
    //         Ciboulette2PostgresTableField::new(table.id().get_ident().clone(), None, None);
    //     let relating_field = Ciboulette2PostgresRelatingField::new(
    //         table_field.clone(),
    //         main_cte_table.clone(),
    //         rel_details.relation_alias().clone(),
    //         main_cte_table.ciboulette_type().clone(),
    //     );
    //     self.gen_select_cte_final(
    //         &state,
    //         &table,
    //         table.ciboulette_type().clone(),
    //         Some(relating_field),
    //         [].iter(),
    //         matches!(requirement_type, Ciboulette2PostgresResponseType::Object),
    //     )?;
    //     self.buf.write_all(b" INNER JOIN ")?;
    //     self.write_table_info(&main_cte_table)?;
    //     self.buf.write_all(b" ON ")?;
    //     self.insert_ident(&table_field, &table)?;
    //     self.buf.write_all(b" = ")?;
    //     self.insert_ident(
    //         &Ciboulette2PostgresTableField::new(field_id.clone(), None, None),
    //         &main_cte_table,
    //     )?;
    //     Ok(())
    // }

    // /// Generate the table that'll be used to in the query to select one-to-many relationships
    // fn gen_rel_select_tables(
    //     &mut self,
    //     rel_rel_table: &Ciboulette2PostgresTable,
    //     rel_table: &Ciboulette2PostgresTable,
    // ) -> Result<(Ciboulette2PostgresTable, Ciboulette2PostgresTable), Ciboulette2SqlError> {
    //     let rel_cte_rel_data = rel_rel_table.to_cte(
    //         &mut *self,
    //         CIBOULETTE_REL_PREFIX,
    //         CIBOULETTE_REL_DATA_SUFFIX,
    //     )?;
    //     let rel_cte_data =
    //         rel_table.to_cte(&mut *self, CIBOULETTE_REL_PREFIX, CIBOULETTE_DATA_SUFFIX)?;
    //     Ok((rel_cte_rel_data, rel_cte_data))
    // }
}
