use super::*;
use getset::CopyGetters;

#[derive(Debug, Clone, Getters, CopyGetters)]
/// State that'll be shared by the query builder during the whole process,
/// allowing to pass fewer arguments per functions
pub(crate) struct Ciboulette2PostgresBuilderState<'store, 'request> {
    /// The resource store
    #[getset(get_copy = "pub")]
    store: &'store CibouletteStore,
    /// The table store, converting resource to underlying Postgres table
    #[getset(get_copy = "pub")]
    table_store: &'store Ciboulette2PostgresTableStore,
    /// The path of the query
    #[getset(get_copy = "pub")]
    path: &'request CiboulettePath<'request>,
    /// The query of the request
    #[getset(get_copy = "pub")]
    query: &'request CibouletteQueryParameters<'request>,
    /// An map of the resource to be included and how to sort the main data
    #[getset(get = "pub")]
    inclusion_map: BTreeMap<
        Vec<CibouletteResourceRelationshipDetails>,
        (
            Ciboulette2PostgresResponseType,
            Vec<CibouletteSortingElement>,
        ),
    >,
    /// The main resource type
    #[getset(get = "pub")]
    main_type: Arc<CibouletteResourceType>,
    /// The main resource table
    #[getset(get = "pub")]
    main_table: Arc<Ciboulette2PostgresTable>,
    /// The main resource expected response type
    #[getset(get_copy = "pub")]
    expected_response_type: Ciboulette2PostgresResponseType,
}

impl<'store, 'request> Ciboulette2PostgresBuilderState<'store, 'request>
where
    'store: 'request,
{
    /// Build the inclusion map, merging the include list and the sorting list
    /// to create a single map
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
