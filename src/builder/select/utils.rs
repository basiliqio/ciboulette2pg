use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(crate) fn gen_sort_inner_joins(
        mut buf: &mut Ciboulette2PostgresBuf,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        rel_table: &Ciboulette2PostgresTableSettings<'a>,
        main_table: &Ciboulette2PostgresTableSettings<'a>,
        main_cte_table: &Ciboulette2PostgresTableSettings<'a>,
        opt: &CibouletteRelationshipOption<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        match opt {
            CibouletteRelationshipOption::ManyDirect(opt) => {
                let rel_rel_table = ciboulette_table_store.get(opt.resource().name())?;
                let main_cte_table_id = Ciboulette2SqlAdditionalField::new(
                    Ciboulette2PostgresTableField::from(main_table.id()),
                    Ciboulette2SqlAdditionalFieldType::MainIdentifier,
                )?;
                let rel_cte_table_id = Ciboulette2SqlAdditionalField::new(
                    Ciboulette2PostgresTableField::from(rel_table.id()),
                    Ciboulette2SqlAdditionalFieldType::MainIdentifier,
                )?;
                buf.write_all(b" INNER JOIN ")?;
                Self::write_table_info_inner(&mut buf, &rel_rel_table)?;
                buf.write_all(b" ON ")?;
                Self::insert_ident_inner(
                    &mut buf,
                    &Ciboulette2PostgresTableField::new_owned(
                        Ciboulette2PostgresSafeIdent::try_from(opt.to().as_str())?,
                        None,
                        None,
                    ),
                    rel_rel_table,
                    None,
                )?;
                buf.write_all(b" = ")?;
                Self::insert_ident_inner(
                    buf,
                    &Ciboulette2PostgresTableField::new_ref(main_cte_table_id.name(), None, None),
                    &main_cte_table,
                    None,
                )?;
                buf.write_all(b" INNER JOIN ")?;
                Self::write_table_info_inner(buf, &rel_table)?;
                buf.write_all(b" ON ")?;
                Self::insert_ident_inner(
                    buf,
                    &Ciboulette2PostgresTableField::new_ref(rel_cte_table_id.name(), None, None),
                    rel_table,
                    None,
                )?;
                buf.write_all(b" = ")?;
                Self::insert_ident_inner(
                    buf,
                    &Ciboulette2PostgresTableField::new_owned(
                        Ciboulette2PostgresSafeIdent::try_from(opt.from().as_str())?,
                        None,
                        None,
                    ),
                    rel_rel_table,
                    None,
                )?;
            }
            CibouletteRelationshipOption::One(opt) => {
                buf.write_all(b" INNER JOIN ")?;
                Self::write_table_info_inner(buf, &main_table)?;
                buf.write_all(b" ON ")?;
                Self::insert_ident_inner(
                    buf,
                    &Ciboulette2PostgresTableField::new_ref(
                        main_table.id().get_ident(),
                        None,
                        None,
                    ),
                    main_table,
                    None,
                )?;
                buf.write_all(b" = ")?;
                Self::insert_ident_inner(
                    buf,
                    &Ciboulette2PostgresTableField::new_ref(
                        main_cte_table.id().get_ident(),
                        None,
                        None,
                    ),
                    main_cte_table,
                    None,
                )?;
                buf.write_all(b" INNER JOIN ")?;
                Self::write_table_info_inner(buf, &rel_table)?;
                buf.write_all(b" ON ")?;
                Self::insert_ident_inner(
                    buf,
                    &Ciboulette2PostgresTableField::new_ref(rel_table.id().get_ident(), None, None),
                    rel_table,
                    None,
                )?;
                buf.write_all(b" = ")?;
                Self::insert_ident_inner(
                    buf,
                    &Ciboulette2PostgresTableField::new_owned(
                        Ciboulette2PostgresSafeIdent::try_from(opt.key().as_str())?,
                        None,
                        None,
                    ),
                    main_table,
                    None,
                )?;
            }
            _ => {
                return Err(Ciboulette2SqlError::UnknownError);
            }
        }
        Ok(())
    }
    pub(crate) fn gen_cte_for_sort(
        &mut self,
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        query: &'a CibouletteQueryParameters<'a>,
        main_type: &'a CibouletteResourceType<'a>,
        main_table: &Ciboulette2PostgresTableSettings<'a>,
        main_cte_data: &Ciboulette2PostgresTableSettings<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        for (type_, sorting_elements) in query.sorting_map().iter() {
            let table = ciboulette_table_store.get(type_.name())?;
            if table == main_table || table == main_cte_data {
                continue;
            }
            match self.working_tables.get(&table) {
                Some(_cte_table) => continue,
                None => {
                    let mut fields: Vec<Ciboulette2PostgresTableField> =
                        Vec::with_capacity(sorting_elements.len());
                    let (_, opt) = ciboulette_store
                        .get_rel(main_type.name().as_str(), type_.name().as_str())?;
                    for el in sorting_elements.iter() {
                        fields.push(Ciboulette2PostgresTableField::new_owned(
                            Ciboulette2PostgresSafeIdent::try_from(el.field().as_ref())?,
                            None,
                            None,
                        ));
                    }
                    let table_cte =
                        table.to_cte(Cow::Owned(format!("cte_{}_data", table.name())))?;
                    self.write_table_info(&table_cte)?;
                    self.buf.write_all(b" AS (SELECT ")?;
                    self.insert_ident(
                        &Ciboulette2PostgresTableField::new_ref(
                            main_cte_data.id().get_ident(),
                            None,
                            None,
                        ),
                        main_cte_data,
                    )?;
                    if !fields.is_empty() {
                        self.buf.write_all(b", ")?;
                    }
                    self.write_list(&fields, &table, false, Self::insert_ident)?;
                    self.buf.write_all(b" FROM ")?;
                    self.write_table_info(&main_cte_data)?;
                    Self::gen_sort_inner_joins(
                        &mut self.buf,
                        &ciboulette_table_store,
                        &table,
                        &main_table,
                        &main_cte_data,
                        &opt,
                    )?;
                }
            }
        }
        Ok(())
    }
    pub(crate) fn gen_sorting_keys(
        &mut self,
        table: &Ciboulette2PostgresTableSettings<'a>,
        type_: &'a CibouletteResourceType<'a>,
        query: &'a CibouletteQueryParameters<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        if let Some(sorting_arr) = query.sorting_map().get(&type_) {
            for el in sorting_arr {
                self.buf.write_all(b", ")?;
                self.insert_ident(
                    &Ciboulette2PostgresTableField::new_owned(
                        Ciboulette2PostgresSafeIdent::try_from(el.field().as_ref())?,
                        Some(Ciboulette2PostgresSafeIdent::try_from(
                            format!("sort_{}", el.field().as_ref()).as_str(),
                        )?),
                        None,
                    ),
                    table,
                )?;
            }
        }
        Ok(())
    }

    fn handle_additionnal_params<'b, I>(
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

    pub(crate) fn gen_select_cte_final<'b, I>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        table: &Ciboulette2PostgresTableSettings<'a>,
        type_: &'a CibouletteResourceType<'a>,
        additional_fields: I,
        include: bool,
    ) -> Result<(), Ciboulette2SqlError>
    where
        'a: 'b,
        I: Iterator<Item = &'b Ciboulette2SqlAdditionalField<'a>>,
    {
        // SELECT
        self.buf.write_all(b"SELECT ")?;
        // SELECT "schema"."mytable"."id"
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_cow(
                Cow::Borrowed(table.id().get_ident()),
                Some(Cow::Owned(Ciboulette2PostgresSafeIdent::try_from("id")?)),
                Some(Cow::Owned(Ciboulette2PostgresSafeIdent::try_from("TEXT")?)),
            ),
            table,
        )?;
        // SELECT "schema"."mytable"."id",
        self.buf.write_all(b", ")?;
        // SELECT "schema"."mytable"."id", $0
        self.insert_params(
            Ciboulette2SqlValue::Text(Some(Cow::Borrowed(type_.name().as_ref()))), // TODO do better
            table,
        )?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type",
        self.buf.write_all(b"::TEXT AS \"type\", ")?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..)
        self.gen_json_builder(table, type_, state.query(), include)?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM
        self.buf.write_all(b" AS \"data\"")?;
        // if let Some(additional_fields) = additional_fields {
        self.handle_additionnal_params(&state, &table, additional_fields)?;
        // }
        self.gen_sorting_keys(&table, &type_, &state.query())?;
        self.buf.write_all(b" FROM ")?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM "schema"."other_table"
        self.write_table_info(table)?;
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

    pub(crate) fn finish_request(
        &mut self,
        state: Ciboulette2PostgresBuilderState<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        let main_cte_table = self.working_tables.get(state.main_table()).ok_or_else(|| {
            CibouletteError::UnknownError("Can't find the main_cte_table".to_string())
        })?;
        let mut iter = self.working_tables.values().peekable();
        while let Some(v) = iter.next() {
            // SELECT * FROM
            self.buf.write_all(b"(SELECT ")?;
            Self::insert_ident_inner(
                &mut self.buf,
                &Ciboulette2PostgresTableField::new_ref(&CIBOULETTE_ID_IDENT, None, None),
                v,
                None,
            )?;
            self.buf.write_all(b", ")?;
            Self::insert_ident_inner(
                &mut self.buf,
                &Ciboulette2PostgresTableField::new_ref(&CIBOULETTE_TYPE_IDENT, None, None),
                v,
                None,
            )?;
            self.buf.write_all(b", ")?;
            Self::insert_ident_inner(
                &mut self.buf,
                &Ciboulette2PostgresTableField::new_ref(&CIBOULETTE_DATA_IDENT, None, None),
                v,
                None,
            )?;
            self.buf.write_all(b" FROM ")?;
            // SELECT * FROM "schema"."mytable"
            Self::write_table_info_inner(&mut self.buf, v)?;
            Self::handle_sorting_routine(
                &mut self.buf,
                &state,
                &main_cte_table,
                v,
                &self.working_tables,
            )?;
            self.buf.write_all(b")")?;
            if iter.peek().is_some() {
                // If there's more :
                // SELECT * FROM "schema"."mytable" UNION ALL ...
                self.buf.write_all(b" UNION ALL ")?;
            }
        }
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
