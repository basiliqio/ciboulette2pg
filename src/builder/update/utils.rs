use super::*;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    /// Generate the update params in the form of `"column_1" = $0, "column_2" = $2`
    pub(super) fn gen_update_params(
        &mut self,
        table: &Ciboulette2PostgresTable,
        params: Vec<(ArcStr, Ciboulette2SqlValue<'request>)>,
    ) -> Result<(), Ciboulette2SqlError> {
        let mut iter = params.into_iter().peekable();
        while let Some((n, v)) = iter.next() {
            self.insert_ident_name(
                &Ciboulette2PostgresTableField::new(
                    Ciboulette2PostgresSafeIdent::try_from(n)?,
                    None,
                    None,
                ),
                &table,
            )?;
            self.buf.write_all(b" = ")?;
            self.insert_params(v, &table)?;

            if iter.peek().is_some() {
                self.buf.write_all(b", ")?;
            }
        }
        Ok(())
    }
}
