use super::*;

impl<'request> Ciboulette2PgBuilder<'request> {
    /// Generate the update params in the form of `"column_1" = $0, "column_2" = $2`
    pub(super) fn gen_update_params(
        &mut self,
        table: &Ciboulette2PgTable,
        params: Vec<(ArcStr, Ciboulette2PgValue<'request>)>,
    ) -> Result<(), Ciboulette2PgError> {
        let mut iter = params.into_iter().peekable();
        while let Some((n, v)) = iter.next() {
            self.insert_ident_name(
                &Ciboulette2PgTableField::new(Ciboulette2PgSafeIdent::try_from(n)?, None, None),
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
