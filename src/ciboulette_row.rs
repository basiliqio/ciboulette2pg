use crate::Ciboulette2SqlError;
use getset::CopyGetters;
use getset::Getters;
use serde::Serialize;
use sqlx::FromRow;
use std::borrow::Cow;

#[derive(Clone, Debug, Getters, Serialize)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresRow<'a> {
    pub(crate) id: Cow<'a, str>,
    #[serde(rename = "type")]
    pub(crate) type_: Cow<'a, str>,
    pub(crate) data: Option<&'a serde_json::value::RawValue>, // TODO doesn't make it an option
}

/// Row returned by a query
///
/// Made of the object id, type and its data
#[derive(Clone, Debug, Getters, CopyGetters, sqlx::FromRow)]
pub struct Ciboulette2PostgresRowBuilder<'a> {
    #[getset(get_copy = "pub")]
    id: &'a str,
    #[sqlx(rename = "type")]
    #[getset(get_copy = "pub")]
    type_: &'a str,
    #[getset(get = "pub")]
    data: Option<&'a serde_json::value::RawValue>, // TODO doesn't make it an option
}

impl<'a> Ciboulette2PostgresRowBuilder<'a> {
    /// Extract an [Ciboulette2PostgresRow](Ciboulette2PostgresRow) for a slice of [PgRow](sqlx::postgres::PgRow)
    pub fn from_raw(
        values: &'a [sqlx::postgres::PgRow]
    ) -> Result<Vec<Ciboulette2PostgresRow>, Ciboulette2SqlError> {
        let mut res = Vec::with_capacity(values.len());

        for val in values.iter() {
            res.push(Ciboulette2PostgresRow::from(Self::from_row(val)?));
        }
        Ok(res)
    }
}

impl<'a> From<Ciboulette2PostgresRowBuilder<'a>> for Ciboulette2PostgresRow<'a> {
    fn from(other: Ciboulette2PostgresRowBuilder<'a>) -> Ciboulette2PostgresRow<'a> {
        Ciboulette2PostgresRow {
            id: Cow::Borrowed(other.id),
            type_: Cow::Borrowed(other.type_),
            data: other.data,
        }
    }
}
