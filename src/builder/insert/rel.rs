use super::*;

impl<'request> Ciboulette2PostgresBuilder<'request> {
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
                Ciboulette2PostgresTableField::new(main_key.clone(), None, None),
                Ciboulette2PostgresTableField::new(rel_key.clone(), None, None),
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
            &Ciboulette2PostgresTableField::new(CIBOULETTE_ID_IDENT, Some(main_key.clone()), None),
            main_table,
        )?;
        self.buf.write_all(b", ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new(CIBOULETTE_ID_IDENT, Some(rel_key.clone()), None),
            rel_table,
        )?;
        self.buf.write_all(b" FROM ")?;
        self.write_table_info(main_table)?;
        self.buf.write_all(b", ")?;
        self.write_table_info(rel_table)?;
        self.buf.write_all(b" RETURNING ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new(dest_table.id().get_ident().clone(), None, None),
            dest_table,
        )?;
        self.buf.write_all(b", ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new(main_key.clone(), None, None),
            dest_table,
        )?;
        self.buf.write_all(b", ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new(rel_key.clone(), None, None),
            dest_table,
        )?;
        Ok(())
    }

    /// Handle inserting multiple relationships and selection them afterwards
    pub(super) fn inserts_handle_many_to_many_rels<'store>(
        &mut self,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        main_cte_data: &Ciboulette2PostgresTable,
        rels: &[Ciboulette2PostgresMainResourceRelationships<'request>],
    ) -> Result<(), Ciboulette2SqlError>
    where
        'store: 'request,
    {
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
                    let rel_cte_id = rel_table.to_cte(CIBOULETTE_ID_SUFFIX)?;
                    let rel_cte_insert = rel_table.to_cte(CIBOULETTE_INSERT_SUFFIX)?;
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
