use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(crate) fn gen_select_cte_with_counter(
        &mut self,
        table: &CibouletteTableSettings,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write(b"SELECT ")?;
        self.insert_ident((table.id_name(), Some("id")), table)?;
        self.buf.write(b", ROW_NUMBER() OVER () as \"rn\" FROM")?;
        self.write_table_info(table)?;
        Ok(())
    }

    pub(crate) fn gen_select(
        &mut self,
        table: &CibouletteTableSettings,
        selected_columns: Vec<(&str, Option<&str>)>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write(b"SELECT ")?;
        if selected_columns.is_empty() {
            self.buf.write(b"*")?;
        } else {
            self.write_list(selected_columns, &table, false, Self::insert_ident)?;
        }
        self.buf.write(b" FROM ")?;
        self.write_table_info(table)?;
        Ok(())
    }
}
