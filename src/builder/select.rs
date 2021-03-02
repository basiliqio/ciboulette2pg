use super::*;

const EMPTY_LIST: [Cow<'static, str>; 0] = [];

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(crate) fn gen_select_cte_final(
        &mut self,
        table: &Ciboulette2PostgresTableSettings<'a>,
        type_: &'a CibouletteResourceType<'a>,
        query: &'a CibouletteQueryParameters<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        // SELECT
        self.buf.write_all(b"SELECT ")?;
        // SELECT "schema"."mytable"."id"
        self.insert_ident(&(table.id_name(), Some("id"), Some("TEXT")), table)?;
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
        self.gen_json_builder(table, type_, query)?;
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
        field_id: &str,
    ) -> Result<(), Ciboulette2SqlError> {
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM "schema"."mytable"
        self.gen_select_cte_final(&table, &type_, &query)?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM "schema"."mytable" WHERE
        self.buf.write_all(b" WHERE ")?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM "schema"."mytable" WHERE "schema"."mytable"."id"
        self.insert_ident(&(table.id_name().as_ref(), None, None), &table)?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM "schema"."mytable" WHERE "schema"."mytable"."id" IN (SELECT
        self.buf.write_all(b" IN (SELECT ")?;
        // SELECT "schema"."mytable"."id", $0::TEXT AS "type", JSON_BUILD_OBJECT(..) AS "data" FROM "schema"."mytable" WHERE "schema"."mytable"."id" IN (SELECT "schema"."othertable"."id"
        self.insert_ident(&(field_id, None, None), &main_table)?;
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
                    self.insert_ident(&(el, None, None), &table)?;
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
    ) -> Result<(), Ciboulette2SqlError> {
        match query.sparse().get(type_) {
            Some(fields) => {
                // If there is no sparse field, nothing will be returned
                self.gen_json_builder_routine(
                    table,
                    type_.schema(),
                    type_.name(),
                    fields.iter().map(Cow::as_ref).peekable(),
                )?;
            }
            None => {
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
        };
        Ok(())
    }

    pub(crate) fn gen_union_select_all<'b, I>(
        &mut self,
        tables: I,
    ) -> Result<(), Ciboulette2SqlError>
    where
        I: IntoIterator<Item = &'b Ciboulette2PostgresTableSettings<'b>>,
    {
        let mut iter = tables.into_iter().peekable();
        while let Some(table) = iter.next() {
            // SELECT * FROM
            self.buf.write_all(b"SELECT * FROM ")?;
            // SELECT * FROM "schema"."mytable"
            self.write_table_info(table)?;
            if iter.peek().is_some() {
                // If there's more :
                // SELECT * FROM "schema"."mytable" UNION ALL ...
                self.buf.write_all(b" UNION ALL ")?;
            }
        }
        Ok(())
    }
}
