use super::*;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
/// State that'll be shared by the query builder during the whole process,
/// allowing to pass fewer arguments per functions
pub(crate) struct Ciboulette2PostgresBuilderState<'a> {
    store: &'a CibouletteStore<'a>,
    table_store: &'a Ciboulette2PostgresTableStore<'a>,
    path: &'a CiboulettePath<'a>,
    query: &'a CibouletteQueryParameters<'a>,
    main_type: &'a CibouletteResourceType<'a>,
    main_table: &'a Ciboulette2PostgresTable<'a>,
    expected_response_type: &'a CibouletteResponseRequiredType,
}

impl<'a> Ciboulette2PostgresBuilderState<'a> {
    /// Check if a relationship is needed in the response.
    pub(crate) fn check_if_rel_is_needed(
        &self,
        other: &CibouletteResourceType<'a>,
        x: &CibouletteResourceType<'a>,
        y: &CibouletteResourceType<'a>,
    ) -> Option<CibouletteResponseRequiredType> {
        self.store()
            .get_rel(x.name(), y.name())
            .ok()
            .and_then(|(_rel_other_type, edge_weight)| match edge_weight {
                CibouletteRelationshipOption::ManyToMany(opt) => {
                    if opt.bucket_resource() == other {
                        Some(CibouletteResponseRequiredType::Object)
                    } else {
                        None
                    }
                }
                CibouletteRelationshipOption::ManyToOne(opt)
                | CibouletteRelationshipOption::OneToMany(opt) => {
                    if opt.one_table() == other || opt.many_table() == other {
                        Some(CibouletteResponseRequiredType::Object)
                    } else {
                        None
                    }
                }
                CibouletteRelationshipOption::OneToOne(_) => None,
            })
    }

    /// Create a new state
    pub fn new(
        store: &'a CibouletteStore<'a>,
        table_store: &'a Ciboulette2PostgresTableStore<'a>,
        path: &'a CiboulettePath<'a>,
        query: &'a CibouletteQueryParameters<'a>,
        expected_response_type: &'a CibouletteResponseRequiredType,
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
            expected_response_type,
        })
    }
}
