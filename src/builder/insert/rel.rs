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
}
