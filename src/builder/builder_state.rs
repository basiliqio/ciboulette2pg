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
    /// Check if a relationship is needed in the response.
    pub(crate) fn check_if_rel_is_needed(
        &self,
        other: &CibouletteResourceType,
        x: &CibouletteResourceType,
        y: &CibouletteResourceRelationshipDetails,
    ) -> Option<Ciboulette2PostgresResponseType> {
        match y.relation_option() {
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
        }
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
