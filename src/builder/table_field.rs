use super::*;
use getset::{Getters, Setters};
use std::convert::TryFrom;

/// Represent a field belonging to a table.
///
/// Also contains the alias and the cast to use in the query, if any
#[derive(Debug, Clone, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub(crate) struct Ciboulette2PgTableField {
    /// The field or fields
    pub(crate) name: Ciboulette2PgSafeIdentSelector,
    /// The new alias of the field, if any
    pub(crate) alias: Option<Ciboulette2PgSafeIdent>,
    /// the cast this field should go under
    pub(crate) cast: Option<Ciboulette2PgSafeIdent>,
}

impl Ciboulette2PgTableField {
    pub fn new(
        name: Ciboulette2PgSafeIdentSelector,
        alias: Option<Ciboulette2PgSafeIdent>,
        cast: Option<Ciboulette2PgSafeIdent>,
    ) -> Self {
        Ciboulette2PgTableField { name, alias, cast }
    }
}

impl From<&Ciboulette2PgId> for Ciboulette2PgTableField {
    fn from(id: &Ciboulette2PgId) -> Self {
        Ciboulette2PgTableField {
            name: Ciboulette2PgSafeIdentSelector::Single(Ciboulette2PgSafeIdent::from(
                id.get_ident(),
            )),
            alias: None,
            cast: Some(id.get_type()),
        }
    }
}

impl From<&Vec<Ciboulette2PgId>> for Ciboulette2PgTableField {
    fn from(ids: &Vec<Ciboulette2PgId>) -> Self {
        let mut res = Vec::with_capacity(ids.len());

        for id in ids {
            res.push(id.get_ident().clone());
        }
        let name = Ciboulette2PgSafeIdentSelector::Multi(res);
        Ciboulette2PgTableField {
            name,
            alias: None,
            cast: None,
        }
    }
}

impl TryFrom<&CibouletteSortingElement> for Ciboulette2PgTableField {
    type Error = Ciboulette2PgError;

    fn try_from(id: &CibouletteSortingElement) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PgTableField {
            name: Ciboulette2PgSafeIdentSelector::Single(Ciboulette2PgSafeIdent::try_from(
                id.field().clone(),
            )?),
            alias: None,
            cast: None,
        })
    }
}
