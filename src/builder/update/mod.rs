use super::*;
use crate::graph_walker::main::Ciboulette2PostgresMain;

pub mod main;
pub mod rel;
pub mod utils;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub fn gen_update_normal(
        &mut self,
        table: &Ciboulette2PostgresTableSettings,
        params: Vec<(&str, Ciboulette2SqlValue<'a>)>,
        query: &'a CibouletteUpdateRequest<'a>,
        returning: bool,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write_all(b"UPDATE ")?;
        self.write_table_info(table)?;
        self.buf.write_all(b" SET ")?;
        self.gen_update_params(table, params)?;
        self.buf.write_all(b" WHERE ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new_ref(table.id_name(), None, None),
            &table,
        )?;
        self.buf.write_all(b" = ")?;
        self.insert_params(
            Ciboulette2SqlValue::Text(Some(Cow::Borrowed(query.resource_id().as_ref()))),
            &table,
        )?;
        if returning {
            self.buf.write_all(b" RETURNING *")?;
        }
        Ok(())
    }
}
