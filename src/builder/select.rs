use super::*;

const EMPTY_LIST: [Cow<'static, str>; 0] = [];

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(crate) fn gen_select_cte_with_counter(
        &mut self,
        table: &CibouletteTableSettings,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write(b"SELECT ")?;
        self.insert_ident(&(table.id_name(), Some("id")), table)?;
        self.buf.write(b", ROW_NUMBER() OVER () as \"rn\" FROM")?;
        self.write_table_info(table)?;
        Ok(())
    }

    pub(crate) fn gen_select_cte_final(
        &mut self,
        table: &CibouletteTableSettings,
        type_: &str,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write(b"SELECT ")?;
        self.insert_ident(&(table.id_name(), Some("id")), table)?;
        self.buf.write(b", ROW_NUMBER() OVER () as \"rn\" FROM")?;
        self.write_table_info(table)?;
        Ok(())
    }

    pub(crate) fn gen_select(
        &mut self,
        table: &CibouletteTableSettings,
        selected_columns: Vec<(&str, Option<&str>)>,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write(b"SELECT ")?;
        if selected_columns.is_empty() {
            self.buf.write(b"*")?;
        } else {
            self.write_list(&selected_columns, &table, false, Self::insert_ident)?;
        }
        self.buf.write(b" FROM ")?;
        self.write_table_info(table)?;
        Ok(())
    }

    pub(crate) fn gen_json_builder_routine<'b, I>(
        &mut self,
        table: &CibouletteTableSettings<'a>,
        obj: &'a MessyJsonObject<'a>,
        obj_name: &'b str,
        mut fields: std::iter::Peekable<I>,
    ) -> Result<(), Ciboulette2SqlError>
    where
        I: std::iter::Iterator<Item = &'a str>,
    {
        if fields.peek().is_none() {
            self.buf.write(b"NULL ")?;
            return Ok(());
        }
        self.buf.write(b"JSON_BUILD_OBJECT(")?;
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
                    self.buf.write(b", ")?;
                    self.insert_ident(&(el, None), &table)?;
                }
            }
            if fields.peek().is_some() {
                self.buf.write(b", ")?;
            }
        }
        self.buf.write(b") ")?;
        Ok(())
    }

    pub(crate) fn gen_json_builder(
        &mut self,
        table: &CibouletteTableSettings<'a>,
        type_: &'a CibouletteResourceType<'a>,
        query: &'a CibouletteQueryParameters<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        let sparse = query.sparse().get(type_).map(|x| x.iter().peekable());
        let fields = match query.sparse().get(type_) {
            Some(fields) => {
                self.gen_json_builder_routine(
                    table,
                    type_.schema(),
                    type_.name(),
                    fields.iter().map(Cow::as_ref).peekable(),
                )?;
            }
            None => {
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
        };
        Ok(())
    }
}
