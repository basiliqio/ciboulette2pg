use super::*;
use getset::Getters;
use std::convert::TryFrom;

/// Represent a field belonging to a table.
///
/// Also contains the alias and the cast to use in the query, if any
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub(crate) struct Ciboulette2PostgresTableField<'a> {
    pub(crate) name: Cow<'a, Ciboulette2PostgresSafeIdent<'a>>,
    pub(crate) alias: Option<Cow<'a, Ciboulette2PostgresSafeIdent<'a>>>,
    pub(crate) cast: Option<Cow<'a, Ciboulette2PostgresSafeIdent<'a>>>,
}

impl<'a> Ciboulette2PostgresTableField<'a> {
    pub fn new_owned(
        name: Ciboulette2PostgresSafeIdent<'a>,
        alias: Option<Ciboulette2PostgresSafeIdent<'a>>,
        cast: Option<Ciboulette2PostgresSafeIdent<'a>>,
    ) -> Self {
        Ciboulette2PostgresTableField {
            name: Cow::Owned(name),
            alias: alias.map(Cow::Owned),
            cast: cast.map(Cow::Owned),
        }
    }
    pub fn new_ref(
        name: &'a Ciboulette2PostgresSafeIdent<'a>,
        alias: Option<&'a Ciboulette2PostgresSafeIdent<'a>>,
        cast: Option<&'a Ciboulette2PostgresSafeIdent<'a>>,
    ) -> Self {
        Ciboulette2PostgresTableField {
            name: Cow::Borrowed(name),
            alias: alias.map(Cow::Borrowed),
            cast: cast.map(Cow::Borrowed),
        }
    }

    pub fn new_cow(
        name: Cow<'a, Ciboulette2PostgresSafeIdent<'a>>,
        alias: Option<Cow<'a, Ciboulette2PostgresSafeIdent<'a>>>,
        cast: Option<Cow<'a, Ciboulette2PostgresSafeIdent<'a>>>,
    ) -> Self {
        Ciboulette2PostgresTableField { name, alias, cast }
    }

    pub fn from_additional_field(id: Ciboulette2SqlAdditionalField<'a>) -> Self {
        Ciboulette2PostgresTableField {
            name: id.ident.name,
            alias: Some(Cow::Owned(id.name)),
            cast: None,
        }
    }

    pub fn from_additional_field_with_cast(id: Ciboulette2SqlAdditionalField<'a>) -> Self {
        Ciboulette2PostgresTableField {
            name: Cow::Owned(id.name),
            alias: None,
            cast: None,
        }
    }
}

impl<'a> From<&Ciboulette2PostgresId<'a>> for Ciboulette2PostgresTableField<'a> {
    fn from(id: &Ciboulette2PostgresId<'a>) -> Self {
        Ciboulette2PostgresTableField {
            name: Cow::Owned(Ciboulette2PostgresSafeIdent::from(id.get_ident())),
            alias: None,
            cast: Some(Cow::Owned(Ciboulette2PostgresSafeIdent::from(
                id.get_type(),
            ))),
        }
    }
}

impl<'a> TryFrom<&CibouletteSortingElement<'a>> for Ciboulette2PostgresTableField<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(id: &CibouletteSortingElement<'a>) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresTableField {
            name: Cow::Owned(Ciboulette2PostgresSafeIdent::try_from(id.field().clone())?),
            alias: None,
            cast: None,
        })
    }
}
