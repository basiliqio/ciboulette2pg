use super::*;
use getset::Getters;
use std::convert::TryFrom;

/// Represent a field belonging to a table.
///
/// Also contains the alias and the cast to use in the query, if any
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresTableField {
    /// The name of the field
    pub(crate) name: Ciboulette2PostgresSafeIdent,
    /// The new alias of the field, if any
    pub(crate) alias: Option<Ciboulette2PostgresSafeIdent>,
    /// the cast this field should go under
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
