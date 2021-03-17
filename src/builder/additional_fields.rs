use super::*;
use std::convert::TryFrom;

#[derive(Clone, Debug)]
pub enum Ciboulette2SqlAdditionalFieldType {
    Relationship,
    MainIdentifier,
}

impl Ciboulette2SqlAdditionalFieldType {
    pub(crate) fn to_sql_prefix(&self) -> &str {
        match self {
            Ciboulette2SqlAdditionalFieldType::Relationship => "rel",
            Ciboulette2SqlAdditionalFieldType::MainIdentifier => "",
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
                type_.to_sql_prefix(),
                ident.name()
            ))?,
            ident,
            type_,
        })
    }
}

impl<'a> TryFrom<&Ciboulette2PostgresTableSettings<'a>> for Ciboulette2SqlAdditionalField<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(table: &Ciboulette2PostgresTableSettings<'a>) -> Result<Self, Self::Error> {
        Ciboulette2SqlAdditionalField::new(
            Ciboulette2PostgresTableField::from(table.id()),
            Ciboulette2SqlAdditionalFieldType::MainIdentifier,
        )
    }
}
