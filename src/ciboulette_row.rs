use crate::Ciboulette2SqlError;
use ciboulette::{CibouletteId, CibouletteResourceIdentifier};
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
    pub(crate) related: Option<CibouletteResourceIdentifier<'a>>,
}

/// Row returned by a query
///
/// Made of the object id, type and its data
#[derive(Clone, Debug, Getters, CopyGetters, sqlx::FromRow)]
pub struct Ciboulette2PostgresRowBuilder<'a> {
    #[getset(get_copy = "pub")]
    id: &'a str,
    #[getset(get_copy = "pub")]
    #[sqlx(rename = "type")]
    type_: &'a str,
    #[getset(get = "pub")]
    data: Option<&'a serde_json::value::RawValue>, // TODO doesn't make it an option
    #[getset(get = "pub")]
    related_type: Option<&'a str>,
    #[getset(get = "pub")]
    related_id: Option<CibouletteId<'a>>,
}

impl<'a> Ciboulette2PostgresRowBuilder<'a> {
    /// Extract an [Ciboulette2PostgresRow](Ciboulette2PostgresRow) for a slice of [PgRow](sqlx::postgres::PgRow)
    pub fn from_raw(
        values: &'a [sqlx::postgres::PgRow]
    ) -> Result<Vec<Ciboulette2PostgresRow>, Ciboulette2SqlError>
    where
        CibouletteId<'a>: sqlx::Decode<'a, sqlx::Postgres>,
        CibouletteId<'a>: sqlx::Type<sqlx::Postgres>,
    {
        let mut res = Vec::with_capacity(values.len());

        for val in values.iter() {
            res.push(Ciboulette2PostgresRow::from(Self::from_row(val)?));
        }
        Ok(res)
    }
}

impl<'a> From<Ciboulette2PostgresRowBuilder<'a>> for Ciboulette2PostgresRow<'a> {
    fn from(other: Ciboulette2PostgresRowBuilder<'a>) -> Ciboulette2PostgresRow<'a> {
        let identifier = match (other.related_type, other.related_id) {
            (Some(related_type), Some(related_id)) => Some(CibouletteResourceIdentifier {
                type_: Cow::Borrowed(related_type),
                id: related_id,
                meta: serde_json::Value::Null,
            }),
            _ => None,
        };
        Ciboulette2PostgresRow {
            id: Cow::Borrowed(other.id),
            type_: Cow::Borrowed(other.type_),
            data: other.data,
            related: identifier,
        }
    }
}
