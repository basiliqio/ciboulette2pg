use super::*;
use getset::Getters;
use std::convert::TryFrom;

/// Represent a field belonging to a table.
///
/// Also contains the alias and the cast to use in the query, if any
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresTableField {
    pub(crate) name: Ciboulette2PostgresSafeIdent,
    pub(crate) alias: Option<Ciboulette2PostgresSafeIdent>,
    pub(crate) cast: Option<Ciboulette2PostgresSafeIdent>,
}

impl Ciboulette2PostgresTableField {
    pub fn new(
        name: Ciboulette2PostgresSafeIdent,
        alias: Option<Ciboulette2PostgresSafeIdent>,
        cast: Option<Ciboulette2PostgresSafeIdent>,
    ) -> Self {
        Ciboulette2PostgresTableField { name, alias, cast }
    }
    pub fn from_additional_field(id: Ciboulette2SqlAdditionalField) -> Self {
        Ciboulette2PostgresTableField {
            name: id.ident.name,
            alias: Some(id.name),
            cast: None,
        }
    }

    pub fn from_additional_field_with_cast(id: Ciboulette2SqlAdditionalField) -> Self {
        Ciboulette2PostgresTableField {
            name: id.name,
            alias: None,
            cast: None,
        }
    }
}

impl From<&Ciboulette2PostgresId> for Ciboulette2PostgresTableField {
    fn from(id: &Ciboulette2PostgresId) -> Self {
        Ciboulette2PostgresTableField {
            name: Ciboulette2PostgresSafeIdent::from(id.get_ident()),
            alias: None,
            cast: Some(id.get_type()),
        }
    }
}

impl TryFrom<&CibouletteSortingElement> for Ciboulette2PostgresTableField {
    type Error = Ciboulette2SqlError;

    fn try_from(id: &CibouletteSortingElement) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresTableField {
            name: Ciboulette2PostgresSafeIdent::try_from(id.field().clone())?,
            alias: None,
            cast: None,
        })
    }
}
