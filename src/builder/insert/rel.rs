use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    /// Generate the query to insert a new type relationship
    pub(super) fn gen_rel_insert(
        &mut self,
        dest_table: &Ciboulette2PostgresTable,
        main_key: &Ciboulette2PostgresSafeIdent,
        rel_key: &Ciboulette2PostgresSafeIdent,
        main_table: &Ciboulette2PostgresTable,
        rel_table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write_all(b"INSERT INTO ")?;
        self.write_table_info(dest_table)?;
        self.buf.write_all(b" ")?;
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
        self.gen_rel_insert_sub_select(main_key, main_table, rel_key, rel_table, dest_table)
    }

    /// Generate the sub query that'll be used to select the provided `ids` linking to the main object
    fn gen_rel_insert_sub_select(
        &mut self,
        main_key: &Ciboulette2PostgresSafeIdent,
        main_table: &Ciboulette2PostgresTable,
        rel_key: &Ciboulette2PostgresSafeIdent,
        rel_table: &Ciboulette2PostgresTable,
        dest_table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write_all(b" SELECT ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_cow(
                Cow::Owned(Ciboulette2PostgresSafeIdent::try_from("id")?),
                Some(Cow::Borrowed(main_key)),
                None,
            ),
            main_table,
        )?;
        self.buf.write_all(b", ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_cow(
                Cow::Owned(Ciboulette2PostgresSafeIdent::try_from("id")?),
                Some(Cow::Borrowed(rel_key)),
                None,
            ),
            rel_table,
        )?;
        self.buf.write_all(b" FROM ")?;
        self.write_table_info(main_table)?;
        self.buf.write_all(b", ")?;
        self.write_table_info(rel_table)?;
        self.buf.write_all(b" RETURNING ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(dest_table.id().get_ident(), None, None),
            dest_table,
        )?;
        self.buf.write_all(b", ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(main_key, None, None),
            dest_table,
        )?;
        self.buf.write_all(b", ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(rel_key, None, None),
            dest_table,
        )?;
        Ok(())
    }

    /// Handle inserting multiple relationships and selection them afterwards
    pub(super) fn inserts_handle_many_to_many_rels(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'a>,
        main_cte_data: &Ciboulette2PostgresTable<'a>,
        rels: &[Ciboulette2PostgresMainResourceRelationships<'a>],
    ) -> Result<(), Ciboulette2SqlError> {
        let rel_iter = rels.iter().peekable();
        for Ciboulette2PostgresMainResourceRelationships {
            type_: rel_type,
            rel_opt: bucket,
            values: rel_ids,
        } in rel_iter
        {
            if let Some(rel_ids) = rel_ids {
                if let Ciboulette2PostgresMultiRelationships::ManyToMany(bucket) = bucket {
                    let rel_table = state.table_store().get(rel_type.name().as_str())?;
                    let rel_rel_table = state
                        .table_store()
                        .get(bucket.bucket_resource().name().as_str())?; // FIXME
                    self.buf.write_all(b", ")?;
                    let rel_cte_id =
                        rel_table.to_cte(Cow::Owned(format!("cte_rel_{}_id", rel_table.name())))?;
                    let rel_cte_insert = rel_table
                        .to_cte(Cow::Owned(format!("cte_rel_{}_insert", rel_table.name())))?;
                    self.write_table_info(&rel_cte_id)?;
                    self.buf.write_all(b" AS (VALUES ")?;
                    self.gen_rel_values(rel_ids.clone(), &rel_table, rel_table.id())?; // FIXME The clone
                    self.buf.write_all(b"), ")?;
                    self.write_table_info(&rel_cte_insert)?;
                    self.buf.write_all(b" AS (")?;
                    self.gen_rel_insert(
                        &rel_rel_table,
                        &Ciboulette2PostgresSafeIdent::try_from(bucket.keys_for_type(rel_type)?)?,
                        &Ciboulette2PostgresSafeIdent::try_from(
                            bucket.keys_for_type(state.main_type())?,
                        )?,
                        &main_cte_data,
                        &rel_cte_id,
                    )?;
                    self.buf.write_all(b")")?;
                }
            }
        }
        self.select_multi_rels_routine(
            state,
            main_cte_data,
            rels,
            Ciboulette2PostgresBuilderState::is_needed_all,
        )?;
        Ok(())
    }
}
