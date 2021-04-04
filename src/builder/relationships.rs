use super::*;

/// Represents relationships of a resource type.
/// The values of the relationships to create/update extracted from the request will
/// also be present
#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2SqlQueryRels<'a> {
    multi_rels: Vec<Ciboulette2PostgresMainResourceRelationships<'a>>,
    single_rels_keys: Vec<Ciboulette2PostgresStr<'a>>,
    single_rels_additional_fields: Vec<Ciboulette2SqlAdditionalField<'a>>,
}

impl<'a> Ciboulette2SqlQueryRels<'a> {
    pub fn new(
        type_: Arc<CibouletteResourceType<'a>>,
        single_rels_keys: Vec<Ciboulette2PostgresStr<'a>>,
        multi_rels: Vec<Ciboulette2PostgresMainResourceRelationships<'a>>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let mut single_rels_additional_fields: Vec<Ciboulette2SqlAdditionalField<'a>> =
            Vec::with_capacity(single_rels_keys.len());
        for single_rel in single_rels_keys.iter() {
            single_rels_additional_fields.push(Ciboulette2SqlAdditionalField::new(
                Ciboulette2PostgresTableField::new_owned(
                    Ciboulette2PostgresSafeIdent::try_from(single_rel.clone())?,
                    None,
                    None,
                ),
                Ciboulette2SqlAdditionalFieldType::Relationship,
                type_.clone(),
            )?)
        }
        Ok(Ciboulette2SqlQueryRels {
            single_rels_keys,
            multi_rels,
            single_rels_additional_fields,
        })
    }
}
