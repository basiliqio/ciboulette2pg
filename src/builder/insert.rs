use super::*;
use crate::graph_walker::main::Ciboulette2PostgresMain;
use crate::graph_walker::relationships::Ciboulette2PostgresRelationships;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub fn gen_insert_normal(
        &mut self,
        table: &Ciboulette2PostgresTableSettings,
        params: Vec<(&str, Ciboulette2SqlValue<'a>)>,
        returning: bool,
    ) -> Result<(), Ciboulette2SqlError> {
        // INSERT INTO
        self.buf.write_all(b"INSERT INTO ")?;
        // INSERT INTO "schema"."mytable"
        self.write_table_info(table)?;
        // INSERT INTO "schema"."mytable"
        self.buf.write_all(b" ")?;
        match params.len() {
            0 => {
                // INSERT INTO "schema"."mytable" DEFAULT VALUES
                self.buf.write_all(b"DEFAULT VALUES")?;
            }
            _ => {
                let mut param_ident: Vec<(
                    Ciboulette2PostgresSafeIdent,
                    Option<Ciboulette2PostgresSafeIdent>,
                    Option<Ciboulette2PostgresSafeIdent>,
                )> = Vec::with_capacity(params.len());
                let mut param_value: Vec<Ciboulette2SqlValue<'_>> =
                    Vec::with_capacity(params.len());

                for (n, v) in params.into_iter() {
                    param_ident.push((Ciboulette2PostgresSafeIdent::try_from(n)?, None, None));
                    param_value.push(v);
                }
                // INSERT INTO "schema"."mytable" (..params..)
                self.write_list(&param_ident, &table, true, Self::insert_ident)?;
                // INSERT INTO "schema"."mytable" (..params..) VALUES
                self.buf.write_all(b" VALUES ")?;
                // INSERT INTO "schema"."mytable" (..params..) VALUES (..values..)
                self.write_list(param_value, &table, true, Self::insert_params)?;
            }
        };
        if returning {
            // INSERT INTO "schema"."mytable" (..params..) VALUES (..values..) RETURNING *
            self.buf.write_all(b" RETURNING *")?;
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
                se.buf.write_all(b"(")?;
                se.insert_params(curr, t)?;
                se.buf.write_all(b"::")?;
                se.buf.write_all(type_.as_bytes())?;
                se.buf.write_all(b")")?;
                Ok(())
            },
        )?;
        Ok(())
    }

    pub fn gen_rel_insert(
        &mut self,
        dest_table: &Ciboulette2PostgresTableSettings,
        main_key: &Ciboulette2PostgresSafeIdent,
        rel_key: &Ciboulette2PostgresSafeIdent,
        main_table: &Ciboulette2PostgresTableSettings,
        rel_table: &Ciboulette2PostgresTableSettings,
    ) -> Result<(), Ciboulette2SqlError> {
        // INSERT INTO
        self.buf.write_all(b"INSERT INTO ")?;
        // INSERT INTO "schema"."mytable"
        self.write_table_info(dest_table)?;
        // INSERT INTO "schema"."mytable"
        self.buf.write_all(b" ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key")
        self.write_list(
            [
                (main_key.clone(), None, None),
                (rel_key.clone(), None, None),
            ]
            .iter(),
            &dest_table,
            true,
            Self::insert_ident,
        )?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT
        self.buf.write_all(b" SELECT ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key"
        self.insert_ident(
            &(
                Ciboulette2PostgresSafeIdent::try_from("id")?,
                Some(main_key.clone()),
                None,
            ),
            main_table,
        )?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key",
        self.buf.write_all(b", ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key"
        self.insert_ident(
            &(
                Ciboulette2PostgresSafeIdent::try_from("id")?,
                Some(rel_key.clone()),
                None,
            ),
            rel_table,
        )?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM
        self.buf.write_all(b" FROM ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table"
        self.write_table_info(main_table)?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table",
        self.buf.write_all(b", ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table"
        self.write_table_info(rel_table)?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING
        self.buf.write_all(b" RETURNING ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING "schema"."mytable"."id"
        self.insert_ident(&(dest_table.id_name().clone(), None, None), dest_table)?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING "schema"."mytable"."id",
        self.buf.write_all(b", ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING "schema"."mytable"."id", "schema"."mytable"."main_key"
        self.insert_ident(&(main_key.clone(), None, None), dest_table)?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING "schema"."mytable"."id", "schema"."mytable"."main_key",
        self.buf.write_all(b", ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING "schema"."mytable"."id", "schema"."mytable"."main_key", "schema"."mytable"."rel_key",
        self.insert_ident(&(rel_key.clone(), None, None), dest_table)?;
        Ok(())
    }

    fn gen_insert_rel_routine(
        &mut self,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        request: &'a CibouletteCreateRequest<'a>,
        table_list: &mut Vec<Ciboulette2PostgresTableSettings<'a>>,
        main_cte_data: &Ciboulette2PostgresTableSettings<'a>,
        mut rels: Vec<Ciboulette2PostgresRelationships<'a>>,
    ) -> Result<(), Ciboulette2SqlError> {
        let rel_iter = rels.iter_mut().peekable();
        for Ciboulette2PostgresRelationships {
            type_: rel_type,
            bucket,
            values: rel_ids,
        } in rel_iter
        {
            if let Some(rel_ids) = rel_ids.take() {
                let rel_table = ciboulette_table_store.get(rel_type.name().as_str())?;
                let rel_rel_table =
                    ciboulette_table_store.get(bucket.resource().name().as_str())?;
                self.buf.write_all(b", ")?;
                let rel_cte_id =
                    rel_table.to_cte(Cow::Owned(format!("cte_rel_{}_id", rel_table.name())))?;
                let rel_cte_insert =
                    rel_table.to_cte(Cow::Owned(format!("cte_rel_{}_insert", rel_table.name())))?;
                // "cte_rel_myrel_id"
                self.write_table_info(&rel_cte_id)?;
                // "cte_rel_myrel_id" AS (VALUES
                self.buf.write_all(b" AS (VALUES ")?;
                // "cte_rel_myrel_id" AS (VALUES ($0::type), ($1::type)
                self.gen_rel_values(rel_ids, rel_table.id_type())?;
                // "cte_rel_myrel_id" AS (VALUES ($0::type), ($1::type)),
                self.buf.write_all(b"), ")?;
                // "cte_rel_myrel_id" AS (VALUES ($0::type), ($1::type)), "cte_rel_myrel_insert"
                self.write_table_info(&rel_cte_insert)?;
                // "cte_rel_myrel_id" AS (VALUES ($0::type), ($1::type)), "cte_rel_myrel_insert" AS (
                self.buf.write_all(b" AS (")?;
                // "cte_rel_myrel_id" AS (VALUES ($0::type), ($1::type)), "cte_rel_myrel_insert" AS (insert_stmt)
                self.gen_rel_insert(
                    &rel_rel_table,
                    &Ciboulette2PostgresSafeIdent::try_from(bucket.from().as_str())?,
                    &Ciboulette2PostgresSafeIdent::try_from(bucket.to().as_str())?,
                    &main_cte_data,
                    &rel_cte_id,
                )?;
                self.buf.write_all(b")")?;
            }
        }
        self.gen_select_multi_rel_routine(
            ciboulette_table_store,
            request.query(),
            table_list,
            main_cte_data,
            rels,
        )?;
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
        let main_cte_insert =
            main_table.to_cte(Cow::Owned(format!("cte_{}_insert", main_table.name())))?;
        let main_cte_data =
            main_table.to_cte(Cow::Owned(format!("cte_{}_data", main_table.name())))?;
        table_list.push(main_cte_data.clone());
        // WITH
        se.buf.write_all(b"WITH \n")?;
        // WITH "cte_main_insert"
        se.write_table_info(&main_cte_insert)?;
        // WITH "cte_main_insert" AS (
        se.buf.write_all(b" AS (")?;
        let Ciboulette2PostgresMain {
            insert_values: main_inserts_values,
            single_relationships: main_single_relationships,
        } = crate::graph_walker::main::gen_query(
            &ciboulette_store,
            request.path().main_type(),
            request.data().attributes(),
            request.data().relationships(),
        )?;
        // WITH "cte_main_insert" AS (insert_stmt)
        se.gen_insert_normal(&main_table, main_inserts_values, true)?;
        se.buf.write_all(b"), ")?;
        // WITH "cte_main_insert" AS (insert_stmt), "cte_main_data"
        se.write_table_info(&main_cte_data)?;
        se.buf.write_all(b" AS (")?;
        se.gen_select_cte_final(&main_cte_insert, &main_type, &request.query(), true)?;
        // WITH "cte_main_insert" AS (insert_stmt), "cte_main_data" AS (select_stmt)
        se.buf.write_all(b")")?;
        let rels = crate::graph_walker::relationships::gen_query(
            &ciboulette_store,
            &request.path().main_type(),
            request.data().relationships(),
        )?;

        se.gen_select_single_rel_routine(
            &ciboulette_store,
            &ciboulette_table_store,
            request.query(),
            &mut table_list,
            &main_type,
            &main_cte_insert,
            main_single_relationships,
        )?;

        // WITH "cte_main_insert" AS (insert_stmt), "cte_main_data" AS (select_stmt),
        // * handle every (one-to-many) relationships *
        se.gen_insert_rel_routine(
            &ciboulette_table_store,
            &request,
            &mut table_list,
            &main_cte_data,
            rels,
        )?;
        se.buf.write_all(b" ")?;
        // Aggregate every table using UNION ALL
        se.gen_union_select_all(&table_list)?;
        Ok(se)
    }
}
