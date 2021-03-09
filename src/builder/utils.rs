use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    #[inline]
    pub fn build(self) -> Result<(String, Ciboulette2SqlArguments<'a>), Ciboulette2SqlError> {
        Ok((
            String::from_utf8(self.buf.into_inner()?.into_inner())?,
            self.params,
        ))
    }

    #[inline]
    pub(crate) fn insert_ident(
        &mut self,
        (ident, alias, cast): &(
            Ciboulette2PostgresSafeIdent,
            Option<Ciboulette2PostgresSafeIdent>,
            Option<Ciboulette2PostgresSafeIdent>,
        ),
        table: &Ciboulette2PostgresTableSettings,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(table)?;
        self.buf.write_all(b".")?;
        self.buf.write_all(POSTGRES_QUOTE)?;
        self.buf.write_all(ident.as_bytes())?;
        self.buf.write_all(POSTGRES_QUOTE)?;
        match cast {
            Some(cast) => {
                self.buf.write_all(b"::")?;
                self.buf.write_all(cast.as_bytes())?;
            }
            None => (),
        };
        match alias {
            Some(alias) => {
                self.buf.write_all(b" AS ")?;
                self.buf.write_all(POSTGRES_QUOTE)?;
                self.buf.write_all(alias.as_bytes())?;
                self.buf.write_all(POSTGRES_QUOTE)?;
            }
            None => (),
        };
        Ok(())
    }

    #[inline]
    pub(crate) fn insert_params(
        &mut self,
        param: Ciboulette2SqlValue<'a>,
        _table: &Ciboulette2PostgresTableSettings<'_>,
    ) -> Result<(), Ciboulette2SqlError> {
        let mut buffer = [0u8; 20];
        let old_len = self.params.len();

        self.params.push(param);
        self.buf.write_all(b"$")?;
        self.buf.write_all(old_len.numtoa(10, &mut buffer))?;
        Ok(())
    }

    #[inline]
    pub(crate) fn write_table_info_inner(
        buf: &mut Ciboulette2PostgresBuf,
        table: &Ciboulette2PostgresTableSettings,
    ) -> Result<(), Ciboulette2SqlError> {
        buf.write_all(POSTGRES_QUOTE)?;
        match table.schema() {
            Some(x) => {
                buf.write_all(x.as_bytes())?;
                buf.write_all(b"\".\"")?;
            }
            None => (),
        };
        buf.write_all(table.name().as_bytes())?;
        buf.write_all(POSTGRES_QUOTE)?;
        Ok(())
    }

    #[inline]
    pub(crate) fn write_table_info(
        &mut self,
        table: &Ciboulette2PostgresTableSettings,
    ) -> Result<(), Ciboulette2SqlError> {
        Self::write_table_info_inner(&mut self.buf, table)
    }

    pub(crate) fn write_list<I, F>(
        &mut self,
        arr: I,
        table: &Ciboulette2PostgresTableSettings,
        wrap_in_parenthesis: bool,
        f: F,
    ) -> Result<(), Ciboulette2SqlError>
    where
        I: std::iter::IntoIterator,
        F: for<'r> Fn(
            &'r mut Ciboulette2PostgresBuilder<'a>,
            I::Item,
            &Ciboulette2PostgresTableSettings,
        ) -> Result<(), Ciboulette2SqlError>,
    {
        let mut iter = arr.into_iter().peekable();
        if wrap_in_parenthesis {
            self.buf.write_all(b"(")?;
        }
        while let Some(curr) = iter.next() {
            f(self, curr, &table)?;

            if iter.peek().is_some() {
                self.buf.write_all(b", ")?;
            }
        }
        if wrap_in_parenthesis {
            self.buf.write_all(b")")?;
        }
        Ok(())
    }
}
