use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(crate) fn gen_sort_joins(
        buf: &mut Ciboulette2PostgresBuf,
        rel_table: &Ciboulette2PostgresTableSettings<'a>,
        main_table: &Ciboulette2PostgresTableSettings<'a>,
        main_cte_table: &Ciboulette2PostgresTableSettings<'a>,
        opt: &CibouletteRelationshipOption<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        match opt {
            CibouletteRelationshipOption::One(opt) => {
                let main_cte_table_id = Ciboulette2SqlAdditionalField::new(
                    Ciboulette2PostgresTableField::from(main_table.id()),
                    Ciboulette2SqlAdditionalFieldType::MainIdentifier,
                )?;
                let rel_cte_table_id = Ciboulette2SqlAdditionalField::new(
                    Ciboulette2PostgresTableField::from(rel_table.id()),
                    Ciboulette2SqlAdditionalFieldType::MainIdentifier,
                )?;
                buf.write_all(b" LEFT JOIN ")?;
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
                    &Ciboulette2PostgresTableField::new_ref(main_cte_table_id.name(), None, None),
                    main_cte_table,
                    None,
                )?;
                buf.write_all(b" LEFT JOIN ")?;
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
                        Ciboulette2PostgresSafeIdent::try_from(opt.key().as_str())?,
                        None,
                        None,
                    ),
                    main_table,
                    None,
                )?;
            }
            _ => {
                return Err(Ciboulette2SqlError::SortingByMultiRel(
                    main_table.ciboulette_type().name().clone(),
                    rel_table.ciboulette_type().name().clone(),
                ));
            }
        }
        Ok(())
    }
    pub(crate) fn gen_cte_for_sort(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        main_cte_data: &Ciboulette2PostgresTableSettings<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        for (type_, sorting_elements) in state.query().sorting_map().iter() {
            let table = state.table_store().get(type_.name())?;
            if &table == state.main_table() || table == main_cte_data {
                continue;
            }
            match self.working_tables.get(&table) {
                Some(_cte_table) => continue,
                None => {
                    let mut fields: Vec<Ciboulette2PostgresTableField> =
                        Vec::with_capacity(sorting_elements.len());
                    let (_, opt) = state
                        .store()
                        .get_rel(state.main_type().name().as_str(), type_.name().as_str())?;
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
                    Self::gen_sort_joins(
                        &mut self.buf,
                        &table,
                        state.main_table(),
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
}
