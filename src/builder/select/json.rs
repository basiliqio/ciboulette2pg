use super::*;
use std::ops::Deref;

impl<'request> Ciboulette2PgBuilder<'request> {
    /// Recursive function that walks a [MessyJsonObject](messy_json::MessyJsonObject) and create the final
    /// `JSON_BUILD_OBJECT` in the query
    pub(crate) fn gen_json_builder_routine<I>(
        &mut self,
        table: &Ciboulette2PgTable,
        obj: MessyJsonObject,
        obj_name: ArcStr,
        mut fields: std::iter::Peekable<I>,
    ) -> Result<(), Ciboulette2PgError>
    where
        I: std::iter::Iterator<Item = ArcStr>,
    {
        // If there is nothing, return an empty JSON object
        if fields.peek().is_none() {
            self.buf.write_all(b"NULL::json ")?;
            return Ok(());
        }
        self.buf.write_all(b"JSON_BUILD_OBJECT(")?;
        while let Some(el) = fields.next() {
            match obj
                .properties()
                .get(&*el)
                .ok_or_else(|| CibouletteError::UnknownField(obj_name.to_string(), el.to_string()))?
                .deref()
            {
                MessyJsonInner::Obj(obj) => {
                    self.gen_json_builder_routine(
                        table,
                        obj.clone(),
                        obj_name.clone(),
                        std::iter::empty::<ArcStr>().peekable(),
                    )?;
                }
                _ => {
                    self.insert_params(Ciboulette2PgValue::ArcStr(Some(el.clone())), &table)?;
                    self.buf.write_all(b", ")?;
                    self.insert_ident(
                        &Ciboulette2PgTableField::new(
                            Ciboulette2PgSafeIdentSelector::Single(
                                Ciboulette2PgSafeIdent::try_from(el)?,
                            ),
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
        table: &Ciboulette2PgTable,
        type_: Arc<CibouletteResourceType>,
        query: &'request CibouletteQueryParameters<'request>,
        include: bool,
    ) -> Result<(), Ciboulette2PgError> {
        match (query.sparse().get(&*type_), include) {
            (Some(fields), true) => {
                // If there is no sparse field, nothing will be returned
                self.gen_json_builder_routine(
                    table,
                    type_.schema().clone(),
                    type_.name().clone(),
                    fields.iter().cloned().peekable(),
                )?;
            }
            (None, true) => {
                // If the sparse parameter is omitted, everything is returned
                self.gen_json_builder_routine(
                    table,
                    type_.schema().clone(),
                    type_.name().clone(),
                    type_.schema().properties().keys().cloned().peekable(),
                )?;
            }
            (_, false) => {
                // If the type is not include, return NULL::json
                self.gen_json_builder_routine(
                    table,
                    type_.schema().clone(),
                    type_.name().clone(),
                    vec![].into_iter().peekable(),
                )?;
            }
        };
        Ok(())
    }
}
