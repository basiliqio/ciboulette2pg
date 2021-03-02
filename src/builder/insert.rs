use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub fn gen_insert_normal(
        &mut self,
        table: &Ciboulette2PostgresTableSettings,
        params: Vec<(&str, Ciboulette2SqlValue<'a>)>,
        returning: bool,
    ) -> Result<(), Ciboulette2SqlError> {
        // INSERT INTO
        self.buf.write(b"INSERT INTO ")?;
        // INSERT INTO "schema"."mytable"
        self.write_table_info(table)?;
        // INSERT INTO "schema"."mytable"
        self.buf.write(b" ")?;
        match params.len() {
            0 => {
                // INSERT INTO "schema"."mytable" DEFAULT VALUES
                self.buf.write(b"DEFAULT VALUES")?;
            }
            _ => {
                let mut param_ident: Vec<(&str, Option<&str>)> = Vec::with_capacity(params.len());
                let mut param_value: Vec<Ciboulette2SqlValue<'_>> =
                    Vec::with_capacity(params.len());

                for (n, v) in params.into_iter() {
                    param_ident.push((n, None));
                    param_value.push(v);
                }
                // INSERT INTO "schema"."mytable" (..params..)
                self.write_list(&param_ident, &table, true, Self::insert_ident)?;
                // INSERT INTO "schema"."mytable" (..params..) VALUES
                self.buf.write(b" VALUES ")?;
                // INSERT INTO "schema"."mytable" (..params..) VALUES (..values..)
                self.write_list(param_value, &table, true, Self::insert_params)?;
            }
        };
        if returning {
            // INSERT INTO "schema"."mytable" (..params..) VALUES (..values..) RETURNING *
            self.buf.write(b" RETURNING *")?;
        }
        Ok(())
    }

    pub fn gen_rel_values(
        &mut self,
        ids: Vec<value::Ciboulette2SqlValue<'a>>,
        type_: &str,
    ) -> Result<(), Ciboulette2SqlError> {
        // It's a logic error to have an empty id vector here
        if ids.is_empty() {
            return Err(Ciboulette2SqlError::EmptyRelValue(type_.to_string()));
        }
        // ($x::type), ($x::type), ($x::type)
        self.write_list(
            ids,
            &Ciboulette2PostgresTableSettings::default(),
            false,
            |ref mut se, curr, t| {
                se.buf.write(b"(")?;
                se.insert_params(curr, t)?;
                se.buf.write(b"::")?;
                se.buf.write(type_.as_bytes())?;
                se.buf.write(b")")?;
                Ok(())
            },
        )?;
        Ok(())
    }

    pub fn gen_rel_insert(
        &mut self,
        dest_table: &Ciboulette2PostgresTableSettings,
        main_key: &str,
        rel_key: &str,
        main_table: &Ciboulette2PostgresTableSettings,
        rel_table: &Ciboulette2PostgresTableSettings,
    ) -> Result<(), Ciboulette2SqlError> {
        // INSERT INTO
        self.buf.write(b"INSERT INTO ")?;
        // INSERT INTO "schema"."mytable"
        self.write_table_info(dest_table)?;
        // INSERT INTO "schema"."mytable"
        self.buf.write(b" ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key")
        self.write_list(
            [(main_key, None), (rel_key, None)].iter(),
            &dest_table,
            true,
            Self::insert_ident,
        )?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT
        self.buf.write(b" SELECT ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key"
        self.insert_ident(&("id", Some(main_key)), main_table)?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key",
        self.buf.write(b", ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key"
        self.insert_ident(&("id", Some(rel_key)), rel_table)?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM
        self.buf.write(b" FROM ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table"
        self.write_table_info(main_table)?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table",
        self.buf.write(b", ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table"
        self.write_table_info(rel_table)?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING
        self.buf.write(b" RETURNING ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING "schema"."mytable"."id"
        self.insert_ident(&(dest_table.id_name(), None), dest_table)?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING "schema"."mytable"."id",
        self.buf.write(b", ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING "schema"."mytable"."id", "schema"."mytable"."main_key"
        self.insert_ident(&(main_key, None), dest_table)?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING "schema"."mytable"."id", "schema"."mytable"."main_key",
        self.buf.write(b", ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING "schema"."mytable"."id", "schema"."mytable"."main_key", "schema"."mytable"."rel_key",
        self.insert_ident(&(rel_key, None), dest_table)?;
        Ok(())
    }

    pub fn gen_insert(
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        request: &'a CibouletteCreateRequest<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let mut se = Self::default();
        let mut table_list: Vec<Ciboulette2PostgresTableSettings<'_>> = Vec::with_capacity(128);
        let main_type = request.path().main_type();
        let main_table = ciboulette_table_store.get(main_type.name().as_str())?;
        let main_cte_insert = Ciboulette2PostgresTableSettings::new_cte(
            Cow::from(main_table.id_name().as_ref()),
            Cow::from(main_table.id_type().as_ref()),
            Cow::from(format!("cte_{}_insert", main_table.name())),
        );
        let main_cte_data = Ciboulette2PostgresTableSettings::new_cte(
            Cow::from(main_table.id_name().as_ref()),
            Cow::from(main_table.id_type().as_ref()),
            Cow::from(format!("cte_{}_data", main_table.name())),
        );
        table_list.push(main_cte_data.clone());
        se.buf.write(b"WITH \n")?;
        se.write_table_info(&main_cte_insert)?;
        se.buf.write(b" AS (")?;
        se.gen_insert_normal(
            &main_table,
            crate::graph_walker::creation::main::gen_query_insert(&ciboulette_store, &request)?,
            true,
        )?;
        se.buf.write(b"), ")?;
        se.write_table_info(&main_cte_data)?;
        se.buf.write(b" AS (")?;
        se.gen_select_cte_final(&main_cte_insert, &main_type, &request.query())?;
        se.buf.write(b")")?;
        let mut rel_iter = crate::graph_walker::creation::relationships::gen_query_insert(
            &ciboulette_store,
            &request,
        )?
        .into_iter()
        .peekable();

        while let Some((rel_type, bucket, rel_ids)) = rel_iter.next() {
            se.buf.write(b", ")?;
            let rel_table = ciboulette_table_store.get(rel_type.name().as_str())?;
            let rel_cte_id = Ciboulette2PostgresTableSettings::new_cte(
                Cow::from(rel_table.id_name().as_ref()),
                Cow::from(rel_table.id_type().as_ref()),
                Cow::from(format!("cte_rel_{}_id", rel_table.name())),
            );
            let rel_cte_insert = Ciboulette2PostgresTableSettings::new_cte(
                Cow::from(rel_table.id_name().as_ref()),
                Cow::from(rel_table.id_type().as_ref()),
                Cow::from(format!("cte_rel_{}_insert", rel_table.name())),
            );
            let rel_cte_rel_data = Ciboulette2PostgresTableSettings::new_cte(
                Cow::from(rel_table.id_name().as_ref()),
                Cow::from(rel_table.id_type().as_ref()),
                Cow::from(format!("cte_rel_{}_rel_data", rel_table.name())),
            );
            let rel_cte_data = Ciboulette2PostgresTableSettings::new_cte(
                Cow::from(rel_table.id_name().as_ref()),
                Cow::from(rel_table.id_type().as_ref()),
                Cow::from(format!("cte_rel_{}_data", rel_table.name())),
            );
            se.write_table_info(&rel_cte_id)?;
            se.buf.write(b" AS (VALUES ")?;
            se.gen_rel_values(rel_ids, rel_table.id_type())?;
            se.buf.write(b"), ")?;
            se.write_table_info(&rel_cte_insert)?;
            se.buf.write(b" AS (")?;
            se.gen_rel_insert(
                &rel_table,
                bucket.from().as_str(),
                bucket.to().as_str(),
                &main_cte_data,
                &rel_cte_id,
            )?;
            se.buf.write(b"), ")?;
            se.write_table_info(&rel_cte_rel_data)?;
            se.buf.write(b" AS (")?;
            se.gen_select_cte_final(&rel_cte_insert, &bucket.resource(), &request.query())?;
            se.buf.write(b"), ")?;
            se.write_table_info(&rel_cte_data)?;
            se.buf.write(b" AS (")?;
            se.gen_select_cte_final(&rel_table, &rel_type, &request.query())?;
            se.buf.write(b" IN (SELECT \"id\" FROM ")?;
            se.write_table_info(&rel_cte_id)?;
            se.buf.write(b")")?;
            table_list.push(rel_cte_rel_data);
            table_list.push(rel_cte_data);
        }
        se.buf.write(b" ")?;
        se.gen_union_select_all(&table_list)?;
        Ok(se)
    }
}
