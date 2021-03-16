use getset::Getters;
use serde::Serialize;
use sqlx::FromRow;

use crate::Ciboulette2SqlError;
#[derive(Clone, Debug, Getters, sqlx::FromRow, Serialize)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresRow<'a> {
    id: &'a str,
    #[sqlx(rename = "type")]
    type_: &'a str,
    data: Option<&'a serde_json::value::RawValue>,
}

impl<'a> Ciboulette2PostgresRow<'a> {
    pub fn from_raw(values: &'a [sqlx::postgres::PgRow]) -> Result<Vec<Self>, Ciboulette2SqlError> {
        let mut res = Vec::with_capacity(values.len());

        for val in values.iter() {
            res.push(Self::from_row(val)?);
        }
        Ok(res)
    }
}
