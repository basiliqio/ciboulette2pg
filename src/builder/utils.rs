use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    #[inline]
    pub fn new() -> Self {
        Ciboulette2PostgresBuilder {
            buf: Ciboulette2PostgresBuf::new_ringbuf(std::io::Cursor::new(Vec::new())),
            params: Ciboulette2SqlArguments::with_capacity(128),
        }
    }

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
        (ident, alias): &(&str, Option<&str>),
        table: &CibouletteTableSettings,
    ) -> Result<(), Ciboulette2SqlError> {
        self.write_table_info(table)?;
        self.buf.write(b".")?;
        self.buf.write(POSTGRES_QUOTE)?;
        self.buf.write(ident.as_bytes())?;
        self.buf.write(POSTGRES_QUOTE)?;
        match alias {
            Some(alias) => {
                self.buf.write(b" AS ")?;
                self.buf.write(POSTGRES_QUOTE)?;
                self.buf.write(alias.as_bytes())?;
                self.buf.write(POSTGRES_QUOTE)?;
            }
            None => (),
        };
        Ok(())
    }

    #[inline]
    pub(crate) fn insert_params(
        &mut self,
        param: Ciboulette2SqlValue<'a>,
        _table: &CibouletteTableSettings,
    ) -> Result<(), Ciboulette2SqlError> {
        let mut buffer = [0u8; 20];
        let old_len = self.params.len();

        self.params.push(param);
        self.buf.write(b"$")?;
        self.buf.write(old_len.numtoa(10, &mut buffer))?;
        Ok(())
    }

    #[inline]
    pub(crate) fn write_table_info(
        &mut self,
        table: &CibouletteTableSettings,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write(POSTGRES_QUOTE)?;
        match table.schema() {
            Some(x) => {
                self.buf.write(x.as_bytes())?;
                self.buf.write(b"\".\"")?;
            }
            None => (),
        };
        self.buf.write(table.name.as_bytes())?;
        self.buf.write(POSTGRES_QUOTE)?;
        Ok(())
    }

    pub(crate) fn write_list<I, F>(
        &mut self,
        arr: I,
        table: &CibouletteTableSettings,
        wrap_in_parenthesis: bool,
        f: F,
    ) -> Result<(), Ciboulette2SqlError>
    where
        I: std::iter::IntoIterator,
        F: for<'r> Fn(
            &'r mut Ciboulette2PostgresBuilder<'a>,
            I::Item,
            &CibouletteTableSettings,
        ) -> Result<(), Ciboulette2SqlError>,
    {
        let mut iter = arr.into_iter().peekable();
        if wrap_in_parenthesis == true {
            self.buf.write(b"(")?;
        }
        loop {
            let curr = match iter.next() {
                Some(x) => x,
                None => break,
            };
            f(self, curr, &table)?;

            if iter.peek().is_some() {
                self.buf.write(b", ")?;
            }
        }
        if wrap_in_parenthesis == true {
            self.buf.write(b")")?;
        }
        Ok(())
    }
}
