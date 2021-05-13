use super::*;

impl<'request> Ciboulette2PgBuilder<'request> {
    /// Generate a insert query for `POST` requests
    pub(crate) fn gen_insert_normal(
        &mut self,
        table: &Ciboulette2PgTable,
        params: Vec<(ArcStr, Ciboulette2PgValue<'request>)>,
        returning: bool,
    ) -> Result<(), Ciboulette2PgError> {
        self.buf.write_all(b"INSERT INTO ")?;
        self.write_table_info(table)?;
        self.buf.write_all(b" ")?;
        match params.len() {
            0 => {
                self.buf.write_all(b"DEFAULT VALUES")?; // Inserts default values if none are provided
            }
            _ => {
                self.gen_normal_insert_values(params, table)?;
            }
        };
        if returning {
            self.buf.write_all(b" RETURNING *")?;
        }
        Ok(())
    }

    /// Generate columns name before the "VALUES" and insert the parameters after that
    fn gen_normal_insert_values(
        &mut self,
        params: Vec<(ArcStr, Ciboulette2PgValue<'request>)>,
        table: &Ciboulette2PgTable,
    ) -> Result<(), Ciboulette2PgError> {
        let mut param_ident: Vec<Ciboulette2PgTableField> = Vec::with_capacity(params.len());
        let mut param_value: Vec<Ciboulette2PgValue<'request>> = Vec::with_capacity(params.len());
        for (n, v) in params.into_iter() {
            param_ident.push(Ciboulette2PgTableField::new(
                Ciboulette2PgSafeIdentSelector::Single(Ciboulette2PgSafeIdent::try_from(n)?),
                None,
                None,
            ));
            param_value.push(v);
        }
        self.write_list(&param_ident, &table, true, Self::insert_ident_name)?;
        self.buf.write_all(b" VALUES ")?;
        self.write_list(param_value, &table, true, Self::insert_params)?;
        Ok(())
    }
}
