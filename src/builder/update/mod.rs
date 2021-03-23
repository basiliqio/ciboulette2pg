use super::*;
use crate::graph_walker::main::Ciboulette2PostgresMainResourceInformations;

pub mod main;
pub mod rel;
pub mod utils;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    /// Generate a normal update with a simple `WHERE` selecting a single id
    pub(crate) fn gen_update_normal(
        &mut self,
        table: &Ciboulette2PostgresTable,
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
            &Ciboulette2PostgresTableField::new_ref(table.id().get_ident(), None, None),
            &table,
        )?;
        self.buf.write_all(b" = ")?;
        self.insert_params(Ciboulette2SqlValue::from(query.resource_id()), &table)?;
        if returning {
            self.buf.write_all(b" RETURNING *")?;
        }
        Ok(())
    }

    /// Generate the CTE table for updating an object
    fn gen_update_cte_tables(
        main_type: &'a Ciboulette2PostgresTable<'a>
    ) -> Result<(Ciboulette2PostgresTable<'a>, Ciboulette2PostgresTable<'a>), Ciboulette2SqlError>
    {
        let main_cte_update =
            main_type.to_cte(Cow::Owned(format!("cte_{}_update", main_type.name())))?;
        let main_cte_data =
            main_type.to_cte(Cow::Owned(format!("cte_{}_data", main_type.name())))?;
        Ok((main_cte_update, main_cte_data))
    }

    pub fn gen_update(
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        request: &'a CibouletteUpdateRequest<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        match request.path() {
            CiboulettePath::TypeId(_, _) => {
                Self::gen_update_main(&ciboulette_store, &ciboulette_table_store, &request)
            }
            CiboulettePath::TypeIdRelationship(type_, _, _) => {
                Self::gen_update_rel(&ciboulette_store, &ciboulette_table_store, &request, &type_)
            }
            _ => unreachable!(), // FIXME
        }
    }
}
