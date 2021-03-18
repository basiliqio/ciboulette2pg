use super::*;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresBuilderState<'a> {
    store: &'a CibouletteStore<'a>,
    table_store: &'a Ciboulette2PostgresTableStore<'a>,
    path: &'a CiboulettePath<'a>,
    query: &'a CibouletteQueryParameters<'a>,
    main_type: &'a CibouletteResourceType<'a>,
    main_table: &'a Ciboulette2PostgresTableSettings<'a>,
    expected_response_type: &'a CibouletteResponseRequiredType,
}

impl<'a> Ciboulette2PostgresBuilderState<'a> {
    pub fn is_type_needed(
        &self,
        other: &CibouletteResourceType<'a>,
    ) -> Option<CibouletteResponseRequiredType> {
        match &other == self.main_type() {
            true => Some(**self.expected_response_type()),
            false => None,
        }
        .or_else(|| match self.path() {
            CiboulettePath::Type(x) | CiboulettePath::TypeId(x, _) => match x == &other {
                true => Some(CibouletteResponseRequiredType::Object),
                false => None,
            },
            CiboulettePath::TypeIdRelated(x, _, y) => match x == &other || y == &other {
                true => Some(CibouletteResponseRequiredType::Object),
                false => None,
            },
            CiboulettePath::TypeIdRelationship(x, _, y) => match x == &other || y == &other {
                true => Some(CibouletteResponseRequiredType::Id),
                false => None,
            },
        })
        .or_else(|| match self.query().include().contains(other) {
            true => Some(CibouletteResponseRequiredType::Object),
            false => None,
        })
        .or_else(|| match self.query().sorting_map().contains_key(other) {
            true => Some(CibouletteResponseRequiredType::None),
            false => None,
        })
    }
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
