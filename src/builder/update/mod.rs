use super::*;

pub mod main;
pub mod rel;
pub mod utils;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    /// Generate a normal update with a simple `WHERE` selecting a single id
    pub(crate) fn gen_update_normal(
        &mut self,
        table: &Ciboulette2PostgresTable,
        params: Vec<(ArcStr, Ciboulette2SqlValue<'request>)>,
        query: &'request CibouletteUpdateRequest<'request>,
        returning: bool,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write_all(b"UPDATE ")?;
        self.write_table_info(table)?;
        self.buf.write_all(b" SET ")?;
        self.gen_update_params(table, params)?;
        self.buf.write_all(b" WHERE ")?;
        self.insert_ident(
            &Ciboulette2PostgresTableField::new(table.id().get_ident().clone(), None, None),
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
        main_type: &Ciboulette2PostgresTable
    ) -> Result<(Ciboulette2PostgresTable, Ciboulette2PostgresTable), Ciboulette2SqlError> {
        let main_cte_update = main_type.to_cte(CIBOULETTE_EMPTY_IDENT, CIBOULETTE_UPDATE_SUFFIX)?;
        let main_cte_data = main_type.to_cte(CIBOULETTE_EMPTY_IDENT, CIBOULETTE_DATA_SUFFIX)?;
        Ok((main_cte_update, main_cte_data))
    }

    pub fn gen_update<'store>(
        ciboulette_store: &'store CibouletteStore,
        ciboulette_table_store: &'store Ciboulette2PostgresTableStore,
        request: &'request CibouletteUpdateRequest<'request>,
    ) -> Result<Self, Ciboulette2SqlError>
    where
        'store: 'request,
    {
        match request.path() {
            CiboulettePath::TypeId(_, _) => {
                Self::gen_update_main(&ciboulette_store, &ciboulette_table_store, &request)
            }
            CiboulettePath::TypeIdRelationship(type_, _, rel_details) => Self::gen_update_rel(
                &ciboulette_store,
                &ciboulette_table_store,
                &request,
                type_.clone(),
                rel_details,
            ),
            _ => unreachable!(), // FIXME
        }
    }
}
