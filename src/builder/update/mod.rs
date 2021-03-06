use super::*;

pub mod main;
pub mod rel;
pub mod utils;

impl<'request> Ciboulette2PgBuilder<'request> {
    /// Generate a normal update with a simple `WHERE` selecting a single id
    pub(crate) fn gen_update_normal(
        &mut self,
        table: &Ciboulette2PgTable,
        params: Vec<(ArcStr, Ciboulette2PgValue<'request>)>,
        query: &'request CibouletteUpdateRequest<'request>,
        returning: bool,
    ) -> Result<(), Ciboulette2PgError> {
        self.buf.write_all(b"UPDATE ")?;
        self.write_table_info(table)?;
        self.buf.write_all(b" SET ")?;
        self.gen_update_params(table, params)?;
        self.buf.write_all(b" WHERE ")?;
        self.compare_pkey(&table, query.resource_id())?;
        if returning {
            self.buf.write_all(b" RETURNING *")?;
        }
        Ok(())
    }

    /// Generate the CTE table for updating an object
    fn gen_update_cte_tables(
        &mut self,
        main_type: &Ciboulette2PgTable,
    ) -> Result<(Ciboulette2PgTable, Ciboulette2PgTable), Ciboulette2PgError> {
        let main_cte_update =
            main_type.to_cte(&mut *self, CIBOULETTE_EMPTY_IDENT, CIBOULETTE_UPDATE_SUFFIX)?;
        let main_cte_data =
            main_type.to_cte(&mut *self, CIBOULETTE_EMPTY_IDENT, CIBOULETTE_DATA_SUFFIX)?;
        Ok((main_cte_update, main_cte_data))
    }

    /// Handle a `PATCH` request, updating resource or Many-to-One relationships
    ///
    /// Fails if the relationships is Many-to-Many or One-to-Many
    ///
    /// Panics if the type of the request
    pub fn gen_update<'store>(
        ciboulette_store: &'store CibouletteStore,
        ciboulette_table_store: &'store Ciboulette2PgTableStore,
        request: &'request CibouletteUpdateRequest<'request>,
    ) -> Result<Self, Ciboulette2PgError>
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
            _ => unreachable!(),
        }
    }
}
