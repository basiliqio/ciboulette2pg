use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(super) fn gen_rel_insert(
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
                Ciboulette2PostgresTableField::new_ref(main_key, None, None),
                Ciboulette2PostgresTableField::new_ref(rel_key, None, None),
            ]
            .iter(),
            &dest_table,
            true,
            Self::insert_ident_name,
        )?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT
        self.buf.write_all(b" SELECT ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key"
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_cow(
                Cow::Owned(Ciboulette2PostgresSafeIdent::try_from("id")?),
                Some(Cow::Borrowed(main_key)),
                None,
            ),
            main_table,
        )?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key",
        self.buf.write_all(b", ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key"
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_cow(
                Cow::Owned(Ciboulette2PostgresSafeIdent::try_from("id")?),
                Some(Cow::Borrowed(rel_key)),
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
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(dest_table.id().get_ident(), None, None),
            dest_table,
        )?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING "schema"."mytable"."id",
        self.buf.write_all(b", ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING "schema"."mytable"."id", "schema"."mytable"."main_key"
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(main_key, None, None),
            dest_table,
        )?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING "schema"."mytable"."id", "schema"."mytable"."main_key",
        self.buf.write_all(b", ")?;
        // INSERT INTO "schema"."mytable" ("main_key", "rel_key") SELECT "schema"."main_table"."id" AS "main_key", "schema"."rel_table"."id" AS "rel_key" FROM "schema"."insert_table", "schema"."id_table" RETURNING "schema"."mytable"."id", "schema"."mytable"."main_key", "schema"."mytable"."rel_key",
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(rel_key, None, None),
            dest_table,
        )?;
        Ok(())
    }

    pub(super) fn inserts_handle_muli_rels(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        main_cte_data: &Ciboulette2PostgresTableSettings<'a>,
        rels: &Vec<Ciboulette2PostgresRelationships<'a>>,
    ) -> Result<(), Ciboulette2SqlError> {
        let rel_iter = rels.iter().peekable();
        for Ciboulette2PostgresRelationships {
            type_: rel_type,
            bucket,
            values: rel_ids,
        } in rel_iter
        {
            if let Some(rel_ids) = rel_ids {
                let rel_table = state.table_store().get(rel_type.name().as_str())?;
                let rel_rel_table = state.table_store().get(bucket.resource().name().as_str())?;
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
                self.gen_rel_values(rel_ids.clone(), &rel_table, rel_table.id())?; // FIXME The clone
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
        self.select_multi_rels_routine(state, main_cte_data, rels)?;
        Ok(())
    }
}
