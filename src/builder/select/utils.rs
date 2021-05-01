use super::*;

impl<'request> Ciboulette2PgBuilder<'request> {
    /// Add additional fields to a final CTE select
    pub(super) fn handle_additionnal_params<'store, 'b, I>(
        &mut self,
        table: &Ciboulette2PgTable,
        additional_fields: I,
    ) -> Result<(), Ciboulette2PgError>
    where
        I: Iterator<Item = &'b Ciboulette2PgAdditionalField>,
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
