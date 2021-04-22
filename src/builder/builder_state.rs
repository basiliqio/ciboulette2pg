use super::*;
use getset::CopyGetters;

#[derive(Debug, Clone, Getters, CopyGetters)]
/// State that'll be shared by the query builder during the whole process,
/// allowing to pass fewer arguments per functions
pub(crate) struct Ciboulette2PostgresBuilderState<'store, 'request> {
    #[getset(get_copy = "pub")]
    store: &'store CibouletteStore,
    #[getset(get_copy = "pub")]
    table_store: &'store Ciboulette2PostgresTableStore,
    #[getset(get_copy = "pub")]
    path: &'request CiboulettePath<'request>,
    #[getset(get_copy = "pub")]
    query: &'request CibouletteQueryParameters<'request>,
    #[getset(get = "pub")]
    inclusion_map: BTreeMap<
        Vec<CibouletteResourceRelationshipDetails>,
        (
            Ciboulette2PostgresResponseType,
            Vec<CibouletteSortingElement>,
        ),
    >,
    #[getset(get = "pub")]
    main_type: Arc<CibouletteResourceType>,
    #[getset(get = "pub")]
    main_table: Arc<Ciboulette2PostgresTable>,
    #[getset(get_copy = "pub")]
    expected_response_type: Ciboulette2PostgresResponseType,
}

impl<'store, 'request> Ciboulette2PostgresBuilderState<'store, 'request>
where
    'store: 'request,
{
    fn build_inclusion_map(
        query: &'request CibouletteQueryParameters<'request>
    ) -> BTreeMap<
        Vec<CibouletteResourceRelationshipDetails>,
        (
            Ciboulette2PostgresResponseType,
            Vec<CibouletteSortingElement>,
        ),
    > {
        let mut res: BTreeMap<
            Vec<CibouletteResourceRelationshipDetails>,
            (
                Ciboulette2PostgresResponseType,
                Vec<CibouletteSortingElement>,
            ),
        > = BTreeMap::new();

        for include_param in query.include() {
            res.insert(
                include_param.clone(),
                (Ciboulette2PostgresResponseType::Object, Vec::default()),
            );
        }
        for sort in query.sorting() {
            if let Some(mut x) = res.insert(
                sort.rel_chain().clone(),
                (Ciboulette2PostgresResponseType::None, vec![sort.clone()]),
            ) {
                if let Some((y, z)) = res.get_mut(sort.rel_chain()) {
                    *y = x.0;
                    z.append(&mut x.1);
                }
            }
        }
        res
    }

    /// Create a new state
    pub fn new(
        store: &'store CibouletteStore,
        table_store: &'store Ciboulette2PostgresTableStore,
        path: &'request CiboulettePath<'request>,
        query: &'request CibouletteQueryParameters<'request>,
        expected_response_type: Ciboulette2PostgresResponseType,
    ) -> Result<Self, Ciboulette2SqlError> {
        let main_type = path.main_type().clone();
        let main_table = table_store.get(main_type.name().as_str())?.clone();

        Ok(Ciboulette2PostgresBuilderState {
            inclusion_map: Self::build_inclusion_map(query),
            store,
            table_store,
            path,
            query,
            main_type,
            main_table,
            expected_response_type,
        })
    }
}
