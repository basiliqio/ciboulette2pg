use super::*;

pub mod json;
pub mod main;
pub mod rel;
pub mod related;
pub mod utils;

const EMPTY_LIST: [Cow<'static, str>; 0] = [];

#[derive(Clone, Debug)]
pub enum Ciboulette2SqlAdditionalFieldType {
    Relationship,
}

impl Ciboulette2SqlAdditionalFieldType {
    pub fn as_str(&self) -> &str {
        match self {
            Ciboulette2SqlAdditionalFieldType::Relationship => "rel",
        }
    }
}

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub")]
pub struct Ciboulette2SqlAdditionalField<'a> {
    type_: Ciboulette2SqlAdditionalFieldType,
    ident: Ciboulette2PostgresTableField<'a>,
    name: Ciboulette2PostgresSafeIdent<'a>,
}

impl<'a> Ciboulette2SqlAdditionalField<'a> {
    pub fn new(
        ident: Ciboulette2PostgresTableField<'a>,
        type_: Ciboulette2SqlAdditionalFieldType,
    ) -> Result<Self, Ciboulette2SqlError> {
        Ok(Ciboulette2SqlAdditionalField {
            name: Ciboulette2PostgresSafeIdent::try_from(format!(
                "{}_{}",
                type_.as_str(),
                ident.name()
            ))?,
            ident,
            type_,
        })
    }
}
