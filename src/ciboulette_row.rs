use super::*;
use crate::Ciboulette2SqlError;
use ciboulette::{
    CibouletteId, CibouletteResourceIdentifier, CibouletteResponseElement, CibouletteStore,
};
use getset::Getters;
use serde::Serialize;
use sqlx::FromRow;
use std::borrow::Cow;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<&'a serde_json::value::RawValue>, // TODO doesn't make it an option
    #[serde(skip_serializing_if = "Option::is_none")]
    related_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_id: Option<&'a str>,
}

impl<'a> Ciboulette2PostgresRow<'a> {
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
            res.push(Self::from_row(val)?);
        }
        Ok(res)
    }

    pub fn into_response_elements(
        self,
        store: &'a CibouletteStore<'a>,
    ) -> Result<CibouletteResponseElement<'a, &'a serde_json::value::RawValue>, Ciboulette2SqlError>
    {
        let type_ = store.get_type(&self.type_)?;
        let id = CibouletteIdBuilder::Text(Cow::Borrowed(self.id)).build(type_.id_type())?;
        let identifier = CibouletteResourceIdentifier::new(id, Cow::Borrowed(self.type_));
        let related_identifier = match (self.related_type, self.related_id) {
            (Some(type_), Some(id)) => {
                let related_type = store.get_type(type_)?;
                let related_id =
                    CibouletteIdBuilder::Text(Cow::Borrowed(id)).build(related_type.id_type())?;
                let related_identifier =
                    CibouletteResourceIdentifier::new(related_id, Cow::Borrowed(type_));
                Some(related_identifier)
            }
            _ => None,
        };
        Ok(CibouletteResponseElement::new(
            &store,
            identifier,
            self.data,
            related_identifier,
        )?)
    }
}
