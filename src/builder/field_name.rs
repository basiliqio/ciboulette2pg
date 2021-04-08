use super::*;
use getset::Getters;
use std::convert::TryFrom;

/// Represent a field belonging to a table.
///
/// Also contains the alias and the cast to use in the query, if any
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresTableField<'store> {
    pub(crate) name: Cow<'store, Ciboulette2PostgresSafeIdent<'store>>,
    pub(crate) alias: Option<Cow<'store, Ciboulette2PostgresSafeIdent<'store>>>,
    pub(crate) cast: Option<Cow<'store, Ciboulette2PostgresSafeIdent<'store>>>,
}

impl<'store> Ciboulette2PostgresTableField<'store> {
    pub fn new_owned(
        name: Ciboulette2PostgresSafeIdent<'store>,
        alias: Option<Ciboulette2PostgresSafeIdent<'store>>,
        cast: Option<Ciboulette2PostgresSafeIdent<'store>>,
    ) -> Self {
        Ciboulette2PostgresTableField {
            name: Cow::Owned(name),
            alias: alias.map(Cow::Owned),
            cast: cast.map(Cow::Owned),
        }
    }
    pub fn new_ref(
        name: &'store Ciboulette2PostgresSafeIdent<'store>,
        alias: Option<&'store Ciboulette2PostgresSafeIdent<'store>>,
        cast: Option<&'store Ciboulette2PostgresSafeIdent<'store>>,
    ) -> Self {
        Ciboulette2PostgresTableField {
            name: Cow::Borrowed(name),
            alias: alias.map(Cow::Borrowed),
            cast: cast.map(Cow::Borrowed),
        }
    }

    pub fn new_cow(
        name: Cow<'store, Ciboulette2PostgresSafeIdent<'store>>,
        alias: Option<Cow<'store, Ciboulette2PostgresSafeIdent<'store>>>,
        cast: Option<Cow<'store, Ciboulette2PostgresSafeIdent<'store>>>,
    ) -> Self {
        Ciboulette2PostgresTableField { name, alias, cast }
    }

    pub fn from_additional_field(id: Ciboulette2SqlAdditionalField<'store>) -> Self {
        Ciboulette2PostgresTableField {
            name: id.ident.name,
            alias: Some(Cow::Owned(id.name)),
            cast: None,
        }
    }

    pub fn from_additional_field_with_cast(id: Ciboulette2SqlAdditionalField<'store>) -> Self {
        Ciboulette2PostgresTableField {
            name: Cow::Owned(id.name),
            alias: None,
            cast: None,
        }
    }
}

impl<'store> From<&Ciboulette2PostgresId<'store>> for Ciboulette2PostgresTableField<'store> {
    fn from(id: &Ciboulette2PostgresId<'store>) -> Self {
        Ciboulette2PostgresTableField {
            name: Cow::Owned(Ciboulette2PostgresSafeIdent::from(id.get_ident())),
            alias: None,
            cast: Some(Cow::Owned(Ciboulette2PostgresSafeIdent::from(
                id.get_type(),
            ))),
        }
    }
}

impl<'store> TryFrom<&CibouletteSortingElement<'store>> for Ciboulette2PostgresTableField<'store> {
    type Error = Ciboulette2SqlError;

    fn try_from(id: &CibouletteSortingElement<'store>) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresTableField {
            name: Cow::Owned(Ciboulette2PostgresSafeIdent::try_from(id.field().clone())?),
            alias: None,
            cast: None,
        })
    }
}
