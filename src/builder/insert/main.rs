use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(crate) fn gen_insert_normal(
        &mut self,
        table: &Ciboulette2PostgresTableSettings,
        params: Vec<(&str, Ciboulette2SqlValue<'a>)>,
        returning: bool,
    ) -> Result<(), Ciboulette2SqlError> {
        // INSERT INTO
        self.buf.write_all(b"INSERT INTO ")?;
        // INSERT INTO "schema"."mytable"
        self.write_table_info(table)?;
        // INSERT INTO "schema"."mytable"
        self.buf.write_all(b" ")?;
        match params.len() {
            0 => {
                // INSERT INTO "schema"."mytable" DEFAULT VALUES
                self.buf.write_all(b"DEFAULT VALUES")?;
            }
            _ => {
                let mut param_ident: Vec<(
                    Ciboulette2PostgresSafeIdent,
                    Option<Ciboulette2PostgresSafeIdent>,
                    Option<Ciboulette2PostgresSafeIdent>,
                )> = Vec::with_capacity(params.len());
                let mut param_value: Vec<Ciboulette2SqlValue<'_>> =
                    Vec::with_capacity(params.len());

                for (n, v) in params.into_iter() {
                    param_ident.push((Ciboulette2PostgresSafeIdent::try_from(n)?, None, None));
                    param_value.push(v);
                }
                // INSERT INTO "schema"."mytable" (..params..)
                self.write_list(&param_ident, &table, true, Self::insert_ident)?;
                // INSERT INTO "schema"."mytable" (..params..) VALUES
                self.buf.write_all(b" VALUES ")?;
                // INSERT INTO "schema"."mytable" (..params..) VALUES (..values..)
                self.write_list(param_value, &table, true, Self::insert_params)?;
            }
        };
        if returning {
            // INSERT INTO "schema"."mytable" (..params..) VALUES (..values..) RETURNING *
            self.buf.write_all(b" RETURNING *")?;
        }
        Ok(())
    }
}
