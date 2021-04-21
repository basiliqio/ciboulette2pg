use super::*;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    /// Add additional fields to a final CTE select
    pub(super) fn handle_additionnal_params<'store, 'b, I>(
        &mut self,
        table: &Ciboulette2PostgresTable,
        additional_fields: I,
    ) -> Result<(), Ciboulette2SqlError>
    where
        I: Iterator<Item = &'b Ciboulette2SqlAdditionalField>,
    {
        for field in additional_fields {
            self.buf.write_all(b", ")?;
            self.insert_ident(&field.ident(), table)?;
            self.buf
                .write_all(format!(" AS \"{}\"", field.name()).as_bytes())?;
        }
        Ok(())
    }
}
