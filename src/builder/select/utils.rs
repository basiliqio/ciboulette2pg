use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
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
            match self.included_tables.get(&table) {
                Some(_cte_table) => continue,
                None => {
                    let mut fields: Vec<(
                        Ciboulette2PostgresSafeIdent<'a>,
                        Option<Ciboulette2PostgresSafeIdent<'a>>,
                        Option<Ciboulette2PostgresSafeIdent<'a>>,
                    )> = Vec::with_capacity(sorting_elements.len());
                    let (_, opt) = ciboulette_store
                        .get_rel(main_type.name().as_str(), type_.name().as_str())?;
                    for el in sorting_elements.iter() {
                        fields.push((
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
                        &(main_cte_data.id_name().clone(), None, None),
                        main_cte_data,
                    )?;
                    if !fields.is_empty() {
                        self.buf.write_all(b", ")?;
                    }
                    self.write_list(&fields, &table, false, Self::insert_ident)?;
                    self.buf.write_all(b" FROM ")?;
                    self.write_table_info(&main_cte_data)?;
                    match opt {
                        CibouletteRelationshipOption::ManyDirect(opt) => {
                            let rel_table = ciboulette_table_store.get(opt.resource().name())?;
                            self.buf.write_all(b" INNER JOIN ")?;
                            self.write_table_info(&rel_table)?;
                            self.buf.write_all(b" ON ")?;
                            self.insert_ident(
                                &(
                                    Ciboulette2PostgresSafeIdent::try_from(opt.to().as_str())?,
                                    None,
                                    None,
                                ),
                                rel_table,
                            )?;
                            self.buf.write_all(b" = ")?;
                            self.insert_ident(
                                &(main_cte_data.id_name().clone(), None, None),
                                main_cte_data,
                            )?;
                            self.buf.write_all(b" INNER JOIN ")?;
                            self.write_table_info(&table)?;
                            self.buf.write_all(b" ON ")?;
                            self.insert_ident(&(table.id_name().clone(), None, None), table)?;
                            self.buf.write_all(b" = ")?;
                            self.insert_ident(
                                &(
                                    Ciboulette2PostgresSafeIdent::try_from(opt.from().as_str())?,
                                    None,
                                    None,
                                ),
                                rel_table,
                            )?;
                        }
                        CibouletteRelationshipOption::One(opt) => {
                            self.buf.write_all(b" INNER JOIN ")?;
                            self.write_table_info(&main_table)?;
                            self.buf.write_all(b" ON ")?;
                            self.insert_ident(
                                &(main_table.id_name().clone(), None, None),
                                main_table,
                            )?;
                            self.buf.write_all(b" = ")?;
                            self.insert_ident(
                                &(main_cte_data.id_name().clone(), None, None),
                                main_cte_data,
                            )?;
                            self.buf.write_all(b" INNER JOIN ")?;
                            self.write_table_info(&table)?;
                            self.buf.write_all(b" ON ")?;
                            self.insert_ident(&(table.id_name().clone(), None, None), table)?;
                            self.buf.write_all(b" = ")?;
                            self.insert_ident(
                                &(
                                    Ciboulette2PostgresSafeIdent::try_from(opt.key().as_str())?,
                                    None,
                                    None,
                                ),
                                main_table,
                            )?;
                        }
                        _ => {
                            return Err(Ciboulette2SqlError::UnkownError);
                        }
                    }
                }
            }
        }
        Ok(())
    }
    pub(crate) fn gen_select_cte_final(
        &mut self,
        table: &Ciboulette2PostgresTableSettings<'a>,
        type_: &'a CibouletteResourceType<'a>,
        query: &'a CibouletteQueryParameters<'a>,
        include: bool,
    ) -> Result<(), Ciboulette2SqlError> {
        // SELECT
        self.buf.write_all(b"SELECT ")?;
        // SELECT "schema"."mytable"."id"
        self.insert_ident(
            &(
                table.id_name().clone(),
                Some(Ciboulette2PostgresSafeIdent::try_from("id")?),
                Some(Ciboulette2PostgresSafeIdent::try_from("TEXT")?),
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
        self.gen_json_builder(table, type_, query, include)?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM
        self.buf.write_all(b" AS \"data\" FROM ")?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM "schema"."other_table"
        self.write_table_info(table)?;
        Ok(())
    }

    pub(crate) fn gen_select_cte_single_rel(
        &mut self,
        table: &Ciboulette2PostgresTableSettings<'a>,
        type_: &'a CibouletteResourceType<'a>,
        query: &'a CibouletteQueryParameters<'a>,
        main_table: &Ciboulette2PostgresTableSettings<'a>,
        field_id: &Ciboulette2PostgresSafeIdent<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM "schema"."mytable"
        self.gen_select_cte_final(&table, &type_, &query, query.include().contains(&type_))?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM "schema"."mytable" WHERE
        self.buf.write_all(b" WHERE ")?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM "schema"."mytable" WHERE "schema"."mytable"."id"
        self.insert_ident(&(table.id_name().clone(), None, None), &table)?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM "schema"."mytable" WHERE "schema"."mytable"."id" IN (SELECT
        self.buf.write_all(b" IN (SELECT ")?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM "schema"."mytable" WHERE "schema"."mytable"."id" IN (SELECT "schema"."othertable"."id"
        self.insert_ident(&(field_id.clone(), None, None), &main_table)?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM "schema"."mytable" WHERE "schema"."mytable"."id" IN (SELECT "schema"."othertable"."id" FROM
        self.buf.write_all(b" FROM ")?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM "schema"."mytable" WHERE "schema"."mytable"."id" IN (SELECT "schema"."othertable"."id" FROM "schema"."othertable"
        self.write_table_info(&main_table)?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM "schema"."mytable" WHERE "schema"."mytable"."id" IN (SELECT "schema"."othertable"."id" FROM "schema"."othertable")
        self.buf.write_all(b")")?;
        Ok(())
    }

    pub(crate) fn gen_union_select_all(
        &mut self,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        query: &'a CibouletteQueryParameters<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        let mut iter = self.included_tables.values().peekable();
        while let Some(v) = iter.next() {
            // SELECT * FROM
            self.buf.write_all(b"SELECT * FROM ")?;
            // SELECT * FROM "schema"."mytable"
            Self::write_table_info_inner(&mut self.buf, v)?;
            Self::handle_sorting_routine(
                &mut self.buf,
                &ciboulette_table_store,
                query.sorting_map(),
                v,
                &self.included_tables,
            )?;
            if iter.peek().is_some() {
                // If there's more :
                // SELECT * FROM "schema"."mytable" UNION ALL ...
                self.buf.write_all(b" UNION ALL ")?;
            }
        }
        Ok(())
    }
}
