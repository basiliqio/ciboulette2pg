use super::*;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresBuilderState<'a> {
    store: &'a CibouletteStore<'a>,
    table_store: &'a Ciboulette2PostgresTableStore<'a>,
    path: &'a CiboulettePath<'a>,
    query: &'a CibouletteQueryParameters<'a>,
    main_type: &'a CibouletteResourceType<'a>,
    main_table: &'a Ciboulette2PostgresTableSettings<'a>,
}

impl<'a> Ciboulette2PostgresBuilderState<'a> {
    pub fn new(
        store: &'a CibouletteStore<'a>,
        table_store: &'a Ciboulette2PostgresTableStore<'a>,
        path: &'a CiboulettePath<'a>,
        query: &'a CibouletteQueryParameters<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let main_type = path.main_type();
        let main_table = table_store.get(main_type.name().as_str())?;

        Ok(Ciboulette2PostgresBuilderState {
            store,
            table_store,
            path,
            query,
            main_type,
            main_table,
        })
    }
}
