use super::*;

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub struct Ciboulette2SqlQueryRels<'a> {
    multi_rels: Vec<Ciboulette2PostgresRelationships<'a>>,
    single_rels_keys: Vec<&'a str>,
    single_rels_additional_fields: Vec<Ciboulette2SqlAdditionalField<'a>>,
}

impl<'a> Ciboulette2SqlQueryRels<'a> {
    pub fn new(
        single_rels_keys: Vec<&'a str>,
        multi_rels: Vec<Ciboulette2PostgresRelationships<'a>>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let mut single_rels_additional_fields: Vec<Ciboulette2SqlAdditionalField> =
            Vec::with_capacity(single_rels_keys.len());
        for single_rel in single_rels_keys.iter() {
            single_rels_additional_fields.push(Ciboulette2SqlAdditionalField::new(
                Ciboulette2PostgresTableField::new_owned(
                    Ciboulette2PostgresSafeIdent::try_from(*single_rel)?,
                    None,
                    None,
                ),
                Ciboulette2SqlAdditionalFieldType::Relationship,
            )?)
        }
        Ok(Ciboulette2SqlQueryRels {
            single_rels_keys,
            multi_rels,
            single_rels_additional_fields,
        })
    }
}
