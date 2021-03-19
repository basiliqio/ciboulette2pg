use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    /// Recursive function that walks a [MessyJsonObject](messy_json::MessyJsonObject) and create the final
    /// `JSON_BUILD_OBJECT` in the query
    pub(crate) fn gen_json_builder_routine<'b, I>(
        &mut self,
        table: &Ciboulette2PostgresTableSettings<'_>,
        obj: &'a MessyJsonObject<'a>,
        obj_name: &'b str,
        mut fields: std::iter::Peekable<I>,
    ) -> Result<(), Ciboulette2SqlError>
    where
        I: std::iter::Iterator<Item = &'a str>,
    {
        // If there is nothing, return an empty JSON object
        if fields.peek().is_none() {
            self.buf.write_all(b"NULL::json ")?;
            return Ok(());
        }
        self.buf.write_all(b"JSON_BUILD_OBJECT(")?;
        while let Some(el) = fields.next() {
            match obj.properties().get(el).ok_or_else(|| {
                CibouletteError::UnknownField(obj_name.to_string(), el.to_string())
            })? {
                MessyJson::Obj(obj) => {
                    self.gen_json_builder_routine(
                        table,
                        obj,
                        obj_name,
                        EMPTY_LIST.iter().map(Cow::as_ref).peekable(), // TODO Find a cleaner way to do that
                    )?;
                }
                _ => {
                    self.insert_params(Ciboulette2SqlValue::Text(Some(Cow::Borrowed(el))), &table)?;
                    self.buf.write_all(b", ")?;
                    self.insert_ident(
                        &Ciboulette2PostgresTableField::new_owned(
                            Ciboulette2PostgresSafeIdent::try_from(el)?,
                            None,
                            None,
                        ),
                        &table,
                    )?;
                }
            }
            if fields.peek().is_some() {
                self.buf.write_all(b", ")?;
            }
        }
        self.buf.write_all(b") ")?;
        Ok(())
    }

    /// Generate the function that'll create the final object JSON returned by the database
    pub(crate) fn gen_json_builder(
        &mut self,
        table: &Ciboulette2PostgresTableSettings<'_>,
        type_: &'a CibouletteResourceType<'a>,
        query: &'a CibouletteQueryParameters<'a>,
        include: bool,
    ) -> Result<(), Ciboulette2SqlError> {
        match (query.sparse().get(type_), include) {
            (Some(fields), true) => {
                // If there is no sparse field, nothing will be returned
                self.gen_json_builder_routine(
                    table,
                    type_.schema(),
                    type_.name(),
                    fields.iter().map(Cow::as_ref).peekable(),
                )?;
            }
            (None, true) => {
                // If the sparse parameter is omitted, everything is returned
                self.gen_json_builder_routine(
                    table,
                    type_.schema(),
                    type_.name(),
                    type_
                        .schema()
                        .properties()
                        .keys()
                        .map(|x| x.as_str())
                        .peekable(),
                )?;
            }
            (_, false) => {
                // If the type is not include, return NULL::json
                self.gen_json_builder_routine(
                    table,
                    type_.schema(),
                    type_.name(),
                    vec![].into_iter().peekable(),
                )?;
            }
        };
        Ok(())
    }
}
