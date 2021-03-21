use getset::Getters;
use serde::Serialize;
use sqlx::FromRow;

use crate::Ciboulette2SqlError;

/// Row returned by a query
///
/// Made of the object id, type and its data
#[derive(Clone, Debug, Getters, sqlx::FromRow, Serialize)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresRow<'a> {
    id: &'a str,
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    type_: &'a str,
    data: Option<&'a serde_json::value::RawValue>, // TODO doesn't make it an option
}

impl<'a> Ciboulette2PostgresRow<'a> {
    /// Extract an [Ciboulette2PostgresRow](Ciboulette2PostgresRow) for a slice of [PgRow](sqlx::postgres::PgRow)
    pub fn from_raw(values: &'a [sqlx::postgres::PgRow]) -> Result<Vec<Self>, Ciboulette2SqlError> {
        let mut res = Vec::with_capacity(values.len());

        for val in values.iter() {
            res.push(Self::from_row(val)?);
        }
        Ok(res)
    }
}
