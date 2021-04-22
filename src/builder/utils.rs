use super::*;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    /// Finish building the query, setting the last ';' and then converting the
    /// final query to UTF-8
    #[inline]
    pub fn build(
        mut self
    ) -> Result<(String, Ciboulette2SqlArguments<'request>), Ciboulette2SqlError> {
        self.buf.write_all(b";")?;
        Ok((
            String::from_utf8(self.buf.into_inner()?.into_inner())?, // TODO Is it the best way to handle that ?
            self.params,
        ))
    }

    /// Inserts an identifier to the query.
    ///
    /// In the form `"schema"."table"."ident"[::CAST] [AS "ALIAS"]`
    #[inline]
    pub(crate) fn insert_ident_inner(
        buf: &mut Ciboulette2PostgresBuf,
        field: &Ciboulette2PostgresTableField,
        table: &Ciboulette2PostgresTable,
        force_cast: Option<&'static str>,
    ) -> Result<(), Ciboulette2SqlError> {
        Self::write_table_info_inner(buf, table)?;
        buf.write_all(b".")?;
        buf.write_all(POSTGRES_QUOTE)?;
        field.name().to_writer(&mut *buf)?;
        buf.write_all(POSTGRES_QUOTE)?;
        if let Some(force_cast) = force_cast {
            buf.write_all(b"::")?;
            buf.write_all(force_cast.as_bytes())?;
        } else {
            match field.cast() {
                Some(cast) => {
                    buf.write_all(b"::")?;
                    cast.to_writer(&mut *buf)?;
                }
                None => (),
            };
        }
        match field.alias() {
            Some(alias) => {
                buf.write_all(b" AS ")?;
                buf.write_all(POSTGRES_QUOTE)?;
                alias.to_writer(&mut *buf)?;
                buf.write_all(POSTGRES_QUOTE)?;
            }
            None => (),
        };
        Ok(())
    }

    /// Wrapper for [insert_ident_inner](Self::insert_ident_inner)
    #[inline]
    pub(crate) fn insert_ident(
        &mut self,
        field: &Ciboulette2PostgresTableField,
        table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        Self::insert_ident_inner(&mut self.buf, &field, table, None)
    }

    /// Inserts an identifier name to the query.
    ///
    /// In the form `"ident"`[::CAST] [AS "ALIAS"]
    #[inline]
    pub(crate) fn insert_ident_name(
        &mut self,
        field: &Ciboulette2PostgresTableField,
        _table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write_all(POSTGRES_QUOTE)?;
        field.name().to_writer(&mut self.buf)?;
        self.buf.write_all(POSTGRES_QUOTE)?;
        match field.cast() {
            Some(cast) => {
                self.buf.write_all(b"::")?;
                cast.to_writer(&mut self.buf)?;
            }
            None => (),
        };
        match field.alias() {
            Some(alias) => {
                self.buf.write_all(b" AS ")?;
                self.buf.write_all(POSTGRES_QUOTE)?;
                alias.to_writer(&mut self.buf)?;
                self.buf.write_all(POSTGRES_QUOTE)?;
            }
            None => (),
        };
        Ok(())
    }

    /// Inserts a parameter into the query in the form of `$n` with n
    /// incremented from 1
    #[inline]
    pub(crate) fn insert_params(
        &mut self,
        param: Ciboulette2SqlValue<'request>,
        _table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        let mut buffer = [0u8; 20];
        self.params.push(param);
        let len = self.params.len();

        self.buf.write_all(b"$")?;
        self.buf.write_all(len.numtoa(10, &mut buffer))?;
        Ok(())
    }

    /// Write the table information to the query
    ///
    /// In the form of `"schema"."table"`
    #[inline]
    pub(crate) fn write_table_info_inner(
        buf: &mut Ciboulette2PostgresBuf,
        table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        buf.write_all(POSTGRES_QUOTE)?;
        match table.schema() {
            Some(x) => {
                x.to_writer(&mut *buf)?;
                buf.write_all(b"\".\"")?;
            }
            None => (),
        };
        table.to_writer(&mut *buf)?;
        buf.write_all(POSTGRES_QUOTE)?;
        Ok(())
    }

    /// Wrapper for [write_table_info_inner](Self::write_table_info_inner)
    #[inline]
    pub(crate) fn write_table_info(
        &mut self,
        table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        Self::write_table_info_inner(&mut self.buf, table)
    }

    /// Write a list of something to the query.
    /// The something part will be determined by a callback function.
    /// The result can be wrapped in parenthesis and will be comma-separated
    pub(crate) fn write_list<I, F>(
        &mut self,
        arr: I,
        table: &Ciboulette2PostgresTable,
        wrap_in_parenthesis: bool,
        f: F,
    ) -> Result<(), Ciboulette2SqlError>
    where
        I: std::iter::IntoIterator,
        F: for<'r> Fn(
            &'r mut Ciboulette2PostgresBuilder<'request>,
            I::Item,
            &Ciboulette2PostgresTable,
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
