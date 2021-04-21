use super::*;

/// Informations about the main resource type, extracted from the request
#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresResourceSingleRelationships {
    pub type_: Arc<CibouletteResourceType>,
    pub key: ArcStr,
    pub rel_details: CibouletteResourceRelationshipDetails,
}

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresMultiRelationships<'request> {
    pub type_: Arc<CibouletteResourceType>,
    pub rel_opt: Ciboulette2PostgresMultiRelationshipsType,
    pub rel_details: CibouletteResourceRelationshipDetails,
    pub values: Option<Vec<Ciboulette2SqlValue<'request>>>,
}

/// Extract informations concerning the main resource's one-to-many relationships
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Ciboulette2PostgresMultiRelationshipsType {
    OneToMany(Arc<CibouletteRelationshipOneToManyOption>),
    ManyToMany(Arc<CibouletteRelationshipManyToManyOption>),
}

impl Ciboulette2PostgresMultiRelationshipsType {
    pub fn dest_resource(&self) -> &CibouletteResourceType {
        match self {
            Ciboulette2PostgresMultiRelationshipsType::OneToMany(x) => x.one_table(),
            Ciboulette2PostgresMultiRelationshipsType::ManyToMany(x) => x.bucket_resource(),
        }
    }

    pub fn dest_key(
        &self,
        main_type: &CibouletteResourceType,
    ) -> Result<ArcStr, CibouletteError> {
        match self {
            Ciboulette2PostgresMultiRelationshipsType::OneToMany(x) => {
                Ok(x.many_table_key().clone())
            }
            Ciboulette2PostgresMultiRelationshipsType::ManyToMany(x) => x.keys_for_type(main_type),
        }
    }
}

/// Informations about the main resource type, extracted from the request
#[derive(Clone, Debug, Default, Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub(crate) struct Ciboulette2PostgresResourceInformations<'request> {
    pub values: Vec<(ArcStr, Ciboulette2SqlValue<'request>)>,
    pub single_relationships: Vec<Ciboulette2PostgresResourceSingleRelationships>,
    pub single_relationships_additional_fields: Vec<Ciboulette2SqlAdditionalField>,
    pub multi_relationships: BTreeMap<ArcStr, Ciboulette2PostgresMultiRelationships<'request>>,
}
