use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(super) fn gen_update_params(
        &mut self,
        table: &Ciboulette2PostgresTableSettings,
        params: Vec<(&str, Ciboulette2SqlValue<'a>)>,
    ) -> Result<(), Ciboulette2SqlError> {
        let mut iter = params.into_iter().peekable();
        while let Some((n, v)) = iter.next() {
            self.insert_ident(
                &Ciboulette2PostgresTableField::new_owned(
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
