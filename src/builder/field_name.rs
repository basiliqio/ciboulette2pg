use super::*;
use getset::Getters;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresTableField<'a> {
    name: Cow<'a, Ciboulette2PostgresSafeIdent<'a>>,
    alias: Option<Cow<'a, Ciboulette2PostgresSafeIdent<'a>>>,
    cast: Option<Cow<'a, Ciboulette2PostgresSafeIdent<'a>>>,
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
}
