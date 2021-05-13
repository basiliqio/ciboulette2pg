use super::*;

impl<'request> Ciboulette2PgBuilder<'request> {
    /// Finish building the query, setting the last ';' and then converting the
    /// final query to UTF-8
    #[inline]
    pub fn build(
        mut self
    ) -> Result<(String, Ciboulette2PgArguments<'request>), Ciboulette2PgError> {
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
        buf: &mut Ciboulette2PgBuf,
        field: &Ciboulette2PgTableField,
        table: &Ciboulette2PgTable,
        force_cast: Option<&'static str>,
    ) -> Result<(), Ciboulette2PgError> {
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
        field: &Ciboulette2PgTableField,
        table: &Ciboulette2PgTable,
    ) -> Result<(), Ciboulette2PgError> {
        Self::insert_ident_inner(&mut self.buf, &field, table, None)
    }

    #[inline]
    pub(crate) fn select_main_id_raw(
        &mut self,
        table: &Ciboulette2PgTable,
    ) -> Result<(), Ciboulette2PgError> {
        match table.id().len() {
            1 => {
                let mut table_field = Ciboulette2PgTableField::from(table.id());

                table_field.set_alias(Some(CIBOULETTE_MAIN_IDENTIFIER));

                Self::insert_ident_inner(&mut self.buf, &table_field, table, None)?;
            }
            _ => {
                self.buf.write_all(b"(")?;
                let mut id_iter = table.id().iter().peekable();
                while let Some(id) = id_iter.next() {
                    Self::write_table_info_inner(&mut self.buf, table)?;
                    self.buf.write_all(b".")?;
                    self.buf.write_all(POSTGRES_QUOTE)?;
                    id.get_ident().to_writer(&mut self.buf)?;
                    self.buf.write_all(POSTGRES_QUOTE)?;
                    if id_iter.peek().is_some() {
                        self.buf.write_all(b", ")?;
                    }
                }
                self.buf.write_all(b") AS ")?;
                CIBOULETTE_MAIN_IDENTIFIER.to_writer(&mut self.buf)?;
            }
        };
        Ok(())
    }

    #[inline]
    pub(crate) fn select_main_id_pretty(
        &mut self,
        table: &Ciboulette2PgTable,
    ) -> Result<(), Ciboulette2PgError> {
        match table.id().len() {
            1 => {
                let mut table_field = Ciboulette2PgTableField::from(table.id());

                table_field.set_alias(Some(CIBOULETTE_ID_IDENT));

                Self::insert_ident_inner(&mut self.buf, &table_field, table, Some("TEXT"))?;
            }
            _ => {
                self.buf.write_all(b"CONCAT_WS(',', ")?;
                let mut id_iter = table.id().iter().peekable();
                while let Some(id) = id_iter.next() {
                    Self::write_table_info_inner(&mut self.buf, table)?;
                    self.buf.write_all(b".")?;
                    self.buf.write_all(POSTGRES_QUOTE)?;
                    id.get_ident().to_writer(&mut self.buf)?;
                    self.buf.write_all(POSTGRES_QUOTE)?;
                    if id_iter.peek().is_some() {
                        self.buf.write_all(b", ")?;
                    }
                }
                self.buf.write_all(b") :: TEXT AS ")?;
                CIBOULETTE_ID_IDENT.to_writer(&mut self.buf)?;
            }
        }
        Ok(())
    }

    /// Inserts an identifier name to the query.
    ///
    /// In the form `"ident"`[::CAST] [AS "ALIAS"]
    #[inline]
    pub(crate) fn insert_ident_name(
        &mut self,
        field: &Ciboulette2PgTableField,
        _table: &Ciboulette2PgTable,
    ) -> Result<(), Ciboulette2PgError> {
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
        param: Ciboulette2PgValue<'request>,
        _table: &Ciboulette2PgTable,
    ) -> Result<(), Ciboulette2PgError> {
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
        buf: &mut Ciboulette2PgBuf,
        table: &Ciboulette2PgTable,
    ) -> Result<(), Ciboulette2PgError> {
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
        table: &Ciboulette2PgTable,
    ) -> Result<(), Ciboulette2PgError> {
        Self::write_table_info_inner(&mut self.buf, table)
    }

    /// Write a list of something to the query.
    /// The something part will be determined by a callback function.
    /// The result can be wrapped in parenthesis and will be comma-separated
    pub(crate) fn write_list<I, F>(
        &mut self,
        arr: I,
        table: &Ciboulette2PgTable,
        wrap_in_parenthesis: bool,
        f: F,
    ) -> Result<(), Ciboulette2PgError>
    where
        I: std::iter::IntoIterator,
        F: for<'r> Fn(
            &'r mut Ciboulette2PgBuilder<'request>,
            I::Item,
            &Ciboulette2PgTable,
        ) -> Result<(), Ciboulette2PgError>,
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

    /// Compare the primary key(s) of a table to the provided CibouletteId
    pub(crate) fn compare_pkey(
        &mut self,
        table: &Ciboulette2PgTable,
        id_val: &'request CibouletteIdSelector<'request>,
    ) -> Result<(), Ciboulette2PgError> {
        match table.ciboulette_type().ids() {
            CibouletteIdTypeSelector::Single(id_type) => {
                let mut id_field =
                    Ciboulette2PgTableField::from(&Ciboulette2PgId::try_from(id_type)?);
                id_field.cast = None;
                self.insert_ident(&id_field, &table)?;
                self.buf.write_all(b" = ")?;
                self.insert_params(Ciboulette2PgValue::from(id_val.get(0)?), &table)?
            }
            CibouletteIdTypeSelector::Multi(id_types) => {
                let piter = id_types.iter().peekable().enumerate();
                for (i, id_type) in piter {
                    let mut id_field =
                        Ciboulette2PgTableField::from(&Ciboulette2PgId::try_from(id_type)?);
                    id_field.cast = None;
                    self.insert_ident(&id_field, &table)?;
                    self.buf.write_all(b" = ")?;
                    self.insert_params(Ciboulette2PgValue::from(id_val.get(i)?), &table)?
                }
            }
        }
        Ok(())
    }
}
