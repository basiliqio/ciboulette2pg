use super::*;
use crate::graph_walker::main::Ciboulette2PostgresMain;
use crate::graph_walker::relationships::Ciboulette2PostgresRelationships;
use itertools::Itertools;

const EMPTY_LIST: [Cow<'static, str>; 0] = [];

impl<'a> Ciboulette2PostgresBuilder<'a> {
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

    pub(crate) fn gen_json_builder_routine<'b, I>(
        &mut self,
        table: &Ciboulette2PostgresTableSettings<'_>,
        obj: &'a MessyJsonObject<'a>,
        obj_name: &'b str,
        mut fields: std::iter::Peekable<I>,
    ) -> Result<(), Ciboulette2SqlError>
    where
        I: std::iter::Iterator<Item = &'a str>,
    {
        // If there is nothing, return an empty JSON object
        if fields.peek().is_none() {
            self.buf.write_all(b"NULL::json ")?;
            return Ok(());
        }
        self.buf.write_all(b"JSON_BUILD_OBJECT(")?;
        while let Some(el) = fields.next() {
            match obj.properties().get(el).ok_or_else(|| {
                CibouletteError::UnknownField(obj_name.to_string(), el.to_string())
            })? {
                MessyJson::Obj(obj) => {
                    self.gen_json_builder_routine(
                        table,
                        obj,
                        obj_name,
                        EMPTY_LIST.iter().map(Cow::as_ref).peekable(), // TODO Find a cleaner way to do that
                    )?;
                }
                _ => {
                    self.insert_params(Ciboulette2SqlValue::Text(Some(Cow::Borrowed(el))), &table)?;
                    self.buf.write_all(b", ")?;
                    self.insert_ident(
                        &(Ciboulette2PostgresSafeIdent::try_from(el)?, None, None),
                        &table,
                    )?;
                }
            }
            if fields.peek().is_some() {
                self.buf.write_all(b", ")?;
            }
        }
        self.buf.write_all(b") ")?;
        Ok(())
    }

    pub(crate) fn gen_json_builder(
        &mut self,
        table: &Ciboulette2PostgresTableSettings<'_>,
        type_: &'a CibouletteResourceType<'a>,
        query: &'a CibouletteQueryParameters<'a>,
        include: bool,
    ) -> Result<(), Ciboulette2SqlError> {
        match (query.sparse().get(type_), include) {
            (Some(fields), true) => {
                // If there is no sparse field, nothing will be returned
                self.gen_json_builder_routine(
                    table,
                    type_.schema(),
                    type_.name(),
                    fields.iter().map(Cow::as_ref).peekable(),
                )?;
            }
            (None, true) => {
                // If the sparse parameter is omitted, everything is returned
                self.gen_json_builder_routine(
                    table,
                    type_.schema(),
                    type_.name(),
                    type_
                        .schema()
                        .properties()
                        .keys()
                        .map(|x| x.as_str())
                        .peekable(),
                )?;
            }
            (_, false) => {
                // If the type is not include, return NULL::json
                self.gen_json_builder_routine(
                    table,
                    type_.schema(),
                    type_.name(),
                    vec![].into_iter().peekable(),
                )?;
            }
        };
        Ok(())
    }

    pub(crate) fn gen_union_select_all(
        &mut self,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        sorting_map: &CibouletteSortingMap<'a>,
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
                &sorting_map,
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

    pub(crate) fn gen_select_single_rel_routine(
        &mut self,
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        query: &'a CibouletteQueryParameters<'a>,
        main_type: &'a CibouletteResourceType<'a>,
        main_cte_data: &Ciboulette2PostgresTableSettings<'a>,
        rels: Vec<&'a str>,
    ) -> Result<(), Ciboulette2SqlError> {
        for key in rels.into_iter() {
            self.buf.write_all(b", ")?;
            let rel_table = ciboulette_table_store.get(key)?;
            let rel_table_cte =
                rel_table.to_cte(Cow::Owned(format!("cte_{}_data", rel_table.name())))?;
            let rel_type = main_type.get_relationship(&ciboulette_store, key)?;
            self.write_table_info(&rel_table_cte)?;
            self.buf.write_all(b" AS (")?;
            self.gen_select_cte_single_rel(
                &rel_table,
                &rel_type,
                &query,
                &main_cte_data,
                &Ciboulette2PostgresSafeIdent::try_from(key)?,
            )?;
            self.buf.write_all(b")")?;
            self.included_tables.insert(&rel_table, rel_table_cte);
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
    ) -> Result<CibouletteSortingMap<'a>, Ciboulette2SqlError> {
        let tables: CibouletteSortingMap<'a> =
            query.sorting().iter().into_group_map_by(|x| x.type_);
        for (type_, sorting_elements) in tables.iter() {
            if type_ == &main_type {
                continue;
            }
            let table = ciboulette_table_store.get(type_.name())?;
            if table == main_table {
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
        Ok(tables)
    }

    pub(crate) fn gen_select_multi_rel_routine(
        &mut self,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        query: &'a CibouletteQueryParameters<'a>,
        main_cte_data: &Ciboulette2PostgresTableSettings<'a>,
        rels: Vec<Ciboulette2PostgresRelationships<'a>>,
    ) -> Result<(), Ciboulette2SqlError> {
        let rel_iter = rels.into_iter().peekable();
        for Ciboulette2PostgresRelationships {
            type_: rel_type,
            bucket,
            values: _rel_ids,
        } in rel_iter
        {
            self.buf.write_all(b", ")?;
            let rel_table = ciboulette_table_store.get(rel_type.name().as_str())?;
            let rel_rel_table = ciboulette_table_store.get(bucket.resource().name().as_str())?;
            let rel_cte_rel_data = rel_rel_table
                .to_cte(Cow::Owned(format!("cte_rel_{}_rel_data", rel_table.name())))?;
            let rel_cte_data =
                rel_table.to_cte(Cow::Owned(format!("cte_rel_{}_data", rel_table.name())))?;
            // "cte_rel_myrel_rel_data"
            self.write_table_info(&rel_cte_rel_data)?;
            // "cte_rel_myrel_rel_data" AS (
            self.buf.write_all(b" AS (")?;
            // "cte_rel_myrel_rel_data" AS (select_stmt
            self.gen_select_cte_final(
                &rel_rel_table,
                &bucket.resource(),
                &query,
                query.include().contains(&bucket.resource()),
            )?;
            // "cte_rel_myrel_rel_data" AS (select_stmt WHERE
            self.buf.write_all(b" WHERE ")?;
            // "cte_rel_myrel_rel_data" AS (select_stmt WHERE "schema"."my_rel_rel"."to"
            self.insert_ident(
                &(
                    Ciboulette2PostgresSafeIdent::try_from(bucket.to().as_str())?,
                    None,
                    None,
                ),
                &rel_rel_table,
            )?;
            // "cte_rel_myrel_rel_data" AS (select_stmt WHERE "schema"."my_rel_rel"."to" =
            self.buf.write_all(b" = ")?;
            // "cte_rel_myrel_rel_data" AS (select_stmt WHERE "schema"."my_rel_rel"."to" = "cte_main_data"."myid"
            self.insert_ident(
                &(main_cte_data.id_name().clone(), None, None),
                &main_cte_data,
            )?;
            // "cte_rel_myrel_rel_data" AS (select_stmt WHERE "schema"."my_rel_rel"."to" = "cte_main_data"."myid"),
            self.buf.write_all(b"), ")?;
            self.write_table_info(&rel_cte_data)?;
            self.buf.write_all(b" AS (")?;
            // "cte_rel_myrel_rel_data" AS (select_stmt WHERE "schema"."my_rel_rel"."to" = "cte_main_data"."myid"), "cte_rel_myrel_data" AS (select_stmt)
            self.gen_select_cte_final(
                &rel_table,
                &rel_type,
                &query,
                query.include().contains(&rel_type),
            )?;
            self.buf.write_all(b" WHERE ")?;
            // "cte_rel_myrel_rel_data" AS (select_stmt WHERE "schema"."my_rel_rel"."to" = "cte_main_data"."myid"), "cte_rel_myrel_data" AS (select_stmt) WHERE "schema"."rel_table"."id" IN (SELECT \"id\" FROM
            self.insert_ident(&(rel_table.id_name().clone(), None, None), &rel_table)?;
            // "cte_rel_myrel_rel_data" AS (select_stmt WHERE "schema"."my_rel_rel"."to" = "cte_main_data"."myid"), "cte_rel_myrel_data" AS (select_stmt) WHERE "schema"."rel_table"."id" IN (SELECT \"id\" FROM
            self.buf.write_all(b" IN (SELECT ")?;
            self.insert_ident(
                &(
                    Ciboulette2PostgresSafeIdent::try_from(bucket.from().as_str())?,
                    None,
                    None,
                ),
                &rel_cte_rel_data,
            )?;
            self.buf.write_all(b" FROM ")?;
            self.write_table_info(&rel_cte_rel_data)?;
            // "cte_rel_myrel_rel_data" AS (select_stmt WHERE "schema"."my_rel_rel"."to" = "cte_main_data"."myid"), "cte_rel_myrel_data" AS (select_stmt) WHERE "schema"."rel_table"."id" IN (SELECT \"id\" FROM "cte_rel_myrel_id")
            self.buf.write_all(b"))")?;
            self.included_tables.insert(&rel_table, rel_cte_data);
            self.included_tables
                .insert(&rel_rel_table, rel_cte_rel_data);
        }
        Ok(())
    }

    pub fn gen_select_normal(
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        request: &'a CibouletteReadRequest<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let mut se = Self::default();
        let main_type = request.path().main_type();
        let main_table = ciboulette_table_store.get(main_type.name().as_str())?;
        let main_cte_data =
            main_table.to_cte(Cow::Owned(format!("cte_{}_data", main_table.name())))?;
        // WITH
        se.buf.write_all(b"WITH \n")?;
        // WITH "cte_main_insert"
        se.write_table_info(&main_cte_data)?;
        // WITH "cte_main_insert" AS (
        se.buf.write_all(b" AS (")?;
        se.gen_select_cte_final(&main_table, &main_type, request.query(), true)?;
        match request.path() {
            CiboulettePath::TypeId(_, id)
            | CiboulettePath::TypeIdRelated(_, id, _)
            | CiboulettePath::TypeIdRelationship(_, id, _) => {
                se.buf.write_all(b" WHERE ")?;
                se.insert_ident(&(main_table.id_name().clone(), None, None), &main_table)?;
                se.buf.write_all(b" = ")?;
                se.insert_params(
                    Ciboulette2SqlValue::Text(Some(Cow::Borrowed(id))),
                    &main_table,
                )?;
            }
            _ => (),
        }
        se.buf.write_all(b")")?;
        let Ciboulette2PostgresMain {
            insert_values: _,
            single_relationships: main_single_relationships,
        } = crate::graph_walker::main::gen_query(
            &ciboulette_store,
            request.path().main_type(),
            &None,
            None,
            false,
        )?;

        let rels = crate::graph_walker::relationships::gen_query(
            &ciboulette_store,
            &request.path().main_type(),
            None,
        )?;
        se.gen_select_single_rel_routine(
            &ciboulette_store,
            &ciboulette_table_store,
            request.query(),
            &main_type,
            &main_cte_data,
            main_single_relationships,
        )?;
        se.gen_select_multi_rel_routine(
            &ciboulette_table_store,
            request.query(),
            &main_cte_data,
            rels,
        )?;
        let sorting_map = se.gen_cte_for_sort(
            &ciboulette_store,
            &ciboulette_table_store,
            request.query(),
            &main_type,
            &main_table,
            &main_cte_data,
        )?;
        se.included_tables.insert(&main_table, main_cte_data);
        // Aggregate every table using UNION ALL
        se.gen_union_select_all(&ciboulette_table_store, &sorting_map)?;
        Ok(se)
    }
}
