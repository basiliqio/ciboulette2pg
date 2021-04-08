use super::*;
use getset::CopyGetters;

#[derive(Debug, Clone, Getters, CopyGetters)]
/// State that'll be shared by the query builder during the whole process,
/// allowing to pass fewer arguments per functions
pub(crate) struct Ciboulette2PostgresBuilderState<'store, 'request> {
    #[getset(get_copy = "pub")]
    store: &'store CibouletteStore<'store>,
    #[getset(get_copy = "pub")]
    table_store: &'store Ciboulette2PostgresTableStore<'store>,
    #[getset(get_copy = "pub")]
    path: &'request CiboulettePath<'request, 'store>,
    #[getset(get_copy = "pub")]
    query: &'request CibouletteQueryParameters<'request, 'store>,
    #[getset(get = "pub")]
    main_type: Arc<CibouletteResourceType<'store>>,
    #[getset(get_copy = "pub")]
    main_table: &'store Ciboulette2PostgresTable<'store>,
    #[getset(get = "pub")]
    expected_response_type: Ciboulette2PostgresResponseType,
}

impl<'store, 'request> Ciboulette2PostgresBuilderState<'store, 'request>
where
    'store: 'request,
{
    /// Check if a relationship is needed in the response.
    pub(crate) fn check_if_rel_is_needed(
        &self,
        other: &CibouletteResourceType<'store>,
        x: &CibouletteResourceType<'store>,
        y: &CibouletteResourceType<'store>,
    ) -> Option<Ciboulette2PostgresResponseType> {
        self.store()
            .get_rel(x.name(), y.name())
            .ok()
            .and_then(|(_rel_other_type, edge_weight)| match edge_weight {
                CibouletteRelationshipOption::ManyToMany(opt) => {
                    if opt.bucket_resource().as_ref() == other {
                        Some(Ciboulette2PostgresResponseType::Object)
                    } else {
                        None
                    }
                }
                CibouletteRelationshipOption::ManyToOne(opt)
                | CibouletteRelationshipOption::OneToMany(opt) => {
                    if opt.one_table().as_ref() == other || opt.many_table().as_ref() == other {
                        Some(Ciboulette2PostgresResponseType::Object)
                    } else {
                        None
                    }
                }
            })
    }

    /// Create a new state
    pub fn new(
        store: &'store CibouletteStore<'store>,
        table_store: &'store Ciboulette2PostgresTableStore<'store>,
        path: &'request CiboulettePath<'request, 'store>,
        query: &'request CibouletteQueryParameters<'request, 'store>,
        expected_response_type: Ciboulette2PostgresResponseType,
    ) -> Result<Self, Ciboulette2SqlError> {
        let main_type = path.main_type().clone();
        let main_table = table_store.get(main_type.name().as_str())?;

        Ok(Ciboulette2PostgresBuilderState {
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
