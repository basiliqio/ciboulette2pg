use super::*;

/// Informations about the main resource type, extracted from the request
#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PgResourceSingleRelationships {
    pub type_: Arc<CibouletteResourceType>,
    pub key: ArcStr,
    pub rel_details: CibouletteResourceRelationshipDetails,
}

/// Informations about an Many-to-Many/One-to-Many relationships, extracted from the request
#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PgMultiRelationships<'request> {
    pub type_: Arc<CibouletteResourceType>,
    pub rel_opt: Ciboulette2PgMultiRelationshipsType,
    pub rel_details: CibouletteResourceRelationshipDetails,
    pub values: Option<Vec<Ciboulette2PgValue<'request>>>,
}

/// Extract informations concerning the main resource's relationships (Many-to-Many/One-to-Many)
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Ciboulette2PgMultiRelationshipsType {
    OneToMany(Arc<CibouletteRelationshipOneToManyOption>),
    ManyToMany(Arc<CibouletteRelationshipManyToManyOption>),
}

/// Informations about the main resource type, extracted from the request
#[derive(Clone, Debug, Default, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub(crate) struct Ciboulette2PgResourceInformations<'request> {
    pub values: Vec<(ArcStr, Ciboulette2PgValue<'request>)>,
    pub single_relationships: Vec<Ciboulette2PgResourceSingleRelationships>,
    pub single_relationships_additional_fields: Vec<Ciboulette2PgAdditionalField>,
    pub multi_relationships: BTreeMap<ArcStr, Ciboulette2PgMultiRelationships<'request>>,
}
