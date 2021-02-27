use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub fn gen_insert_normal(
        &mut self,
        table: &CibouletteTableSettings,
        params: Vec<(&str, Ciboulette2SqlValue<'a>)>,
        returning: bool,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write(b"INSERT INTO ")?;
        self.write_table_info(table)?;
        self.buf.write(b" ")?;
        match params.len() {
            0 => {
                self.buf.write(b"DEFAULT VALUES")?;
            }
            _ => {
                let mut param_ident: Vec<&str> = Vec::with_capacity(params.len());
                let mut param_value: Vec<Ciboulette2SqlValue<'_>> =
                    Vec::with_capacity(params.len());

                for (n, v) in params.into_iter() {
                    param_ident.push(n);
                    param_value.push(v);
                }
                self.write_list(
                    param_ident.into_iter().map(|x| (x, None)),
                    &table,
                    true,
                    Self::insert_ident,
                )?;
                self.buf.write(b" VALUES ")?;
                self.write_list(param_value, &table, true, Self::insert_params)?;
            }
        };
        if returning {
            self.buf.write(b" RETURNING *")?;
        }
        Ok(())
    }
    pub fn gen_rel_values(
        &mut self,
        ids: Vec<value::Ciboulette2SqlValue<'a>>,
        type_: &str,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_list(
            ids,
            &CibouletteTableSettings::default(),
            true,
            |ref mut se, curr, t| {
                se.insert_params(curr, t)?;
                se.buf.write(b"::")?;
                se.buf.write(type_.as_bytes())?;
                Ok(())
            },
        )?;
        Ok(())
    }
}
