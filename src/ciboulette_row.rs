use super::*;
use crate::Ciboulette2SqlError;
use ciboulette::{
    CibouletteId, CibouletteResourceIdentifier, CibouletteResponseElement, CibouletteStore,
};
use getset::Getters;
use serde::Serialize;
use sqlx::FromRow;
use std::{borrow::Cow, usize};

/// Row returned by a query
///
/// Made of the object id, type and its data
#[derive(Clone, Debug, Getters, sqlx::FromRow, Serialize)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresRow<'store> {
    id: &'store str,
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    type_: &'store str,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<&'store serde_json::value::RawValue>, // TODO doesn't make it an option
    #[serde(skip_serializing_if = "Option::is_none")]
    related_type: Option<&'store str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_id: Option<&'store str>,
}

impl<'store> Ciboulette2PostgresRow<'store> {
    /// Extract an [Ciboulette2PostgresRow](Ciboulette2PostgresRow) for a slice of [PgRow](sqlx::postgres::PgRow)
    pub fn from_raw(
        values: &'store [sqlx::postgres::PgRow]
    ) -> Result<Vec<Ciboulette2PostgresRow>, Ciboulette2SqlError>
    where
        CibouletteId<'store>: sqlx::Decode<'store, sqlx::Postgres>,
        CibouletteId<'store>: sqlx::Type<sqlx::Postgres>,
    {
        let mut res = Vec::with_capacity(values.len());

        for val in values.iter() {
            res.push(Self::from_row(val)?);
        }
        Ok(res)
    }

    pub fn build_response_elements<'request, I>(
        rows: I,
        store: &'store CibouletteStore<'store>,
        hint_size: Option<usize>,
    ) -> Result<
        Vec<CibouletteResponseElement<'request, 'store, &'store serde_json::value::RawValue>>,
        Ciboulette2SqlError,
    >
    where
        'store: 'request,
        I: IntoIterator<Item = Ciboulette2PostgresRow<'store>>,
    {
        let mut res: Vec<
            CibouletteResponseElement<'request, 'store, &'store serde_json::value::RawValue>,
        > = Vec::with_capacity(hint_size.unwrap_or_default());

        for row in rows.into_iter() {
            let type_ = store.get_type(&row.type_)?;
            let id = CibouletteIdBuilder::Text(Cow::Borrowed(row.id)).build(type_.id_type())?;
            let identifier = CibouletteResourceIdentifier::new(id, Cow::Borrowed(row.type_));
            let related_identifier = match (row.related_type, row.related_id) {
                (Some(type_), Some(id)) => {
                    let related_type = store.get_type(type_)?;
                    let related_id = CibouletteIdBuilder::Text(Cow::Borrowed(id))
                        .build(related_type.id_type())?;
                    let related_identifier =
                        CibouletteResourceIdentifier::new(related_id, Cow::Borrowed(type_));
                    Some(related_identifier)
                }
                _ => None,
            };
            res.push(CibouletteResponseElement::new(
                &store,
                identifier,
                row.data,
                related_identifier,
            )?);
        }
        Ok(res)
    }
}
