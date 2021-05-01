use super::*;
use getset::Getters;
use std::convert::TryFrom;

/// Represent a field belonging to a table.
///
/// Also contains the alias and the cast to use in the query, if any
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PgTableField {
    /// The name of the field
    pub(crate) name: Ciboulette2PgSafeIdent,
    /// The new alias of the field, if any
    pub(crate) alias: Option<Ciboulette2PgSafeIdent>,
    /// the cast this field should go under
    pub(crate) cast: Option<Ciboulette2PgSafeIdent>,
}

impl Ciboulette2PgTableField {
    pub fn new(
        name: Ciboulette2PgSafeIdent,
        alias: Option<Ciboulette2PgSafeIdent>,
        cast: Option<Ciboulette2PgSafeIdent>,
    ) -> Self {
        Ciboulette2PgTableField { name, alias, cast }
    }
}

impl From<&Ciboulette2PgId> for Ciboulette2PgTableField {
    fn from(id: &Ciboulette2PgId) -> Self {
        Ciboulette2PgTableField {
            name: Ciboulette2PgSafeIdent::from(id.get_ident()),
            alias: None,
            cast: Some(id.get_type()),
        }
    }
}

impl TryFrom<&CibouletteSortingElement> for Ciboulette2PgTableField {
    type Error = Ciboulette2PgError;

    fn try_from(id: &CibouletteSortingElement) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PgTableField {
            name: Ciboulette2PgSafeIdent::try_from(id.field().clone())?,
            alias: None,
            cast: None,
        })
    }
}
