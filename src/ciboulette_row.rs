use super::*;
use crate::Ciboulette2PgError;
use ciboulette::{
    CibouletteId, CibouletteResourceType, CibouletteResponseElement, CibouletteStore,
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
pub struct Ciboulette2PgRow<'rows> {
    /// The id of the resource, casted to TEXT
    id: &'rows str,
    /// The type of the resource beeing returned, casted to TEXT
    /// In case of a relationships, this is the relationships chain
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    type_: &'rows str,
    /// The json formatted data, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<&'rows serde_json::value::RawValue>,
    /// The type id it relates to
    #[serde(skip_serializing_if = "Option::is_none")]
    related_type: Option<&'rows str>,
    /// The id of the resource it relates
    #[serde(skip_serializing_if = "Option::is_none")]
    related_id: Option<&'rows str>,
}

impl<'rows> Ciboulette2PgRow<'rows> {
    /// Extract an [Ciboulette2PgRow](Ciboulette2PgRow) for a slice of [PgRow](sqlx::postgres::PgRow)
    pub fn from_raw(
        values: &'rows [sqlx::postgres::PgRow]
    ) -> Result<Vec<Ciboulette2PgRow>, Ciboulette2PgError>
    where
        CibouletteId<'rows>: sqlx::Decode<'rows, sqlx::Postgres>,
        CibouletteId<'rows>: sqlx::Type<sqlx::Postgres>,
    {
        let mut res = Vec::with_capacity(values.len());

        for val in values.iter() {
            res.push(Self::from_row(val)?);
        }
        Ok(res)
    }

    /// Build the responses elements for the main type from an iterator
    pub fn build_response_elements<I>(
        rows: I,
        store: &CibouletteStore,
        main_type: &Arc<CibouletteResourceType>,
        hint_size: Option<usize>,
    ) -> Result<
        Vec<CibouletteResponseElement<'rows, &'rows serde_json::value::RawValue>>,
        Ciboulette2PgError,
    >
    where
        I: IntoIterator<Item = Ciboulette2PgRow<'rows>>,
    {
        let mut res: Vec<CibouletteResponseElement<'rows, &'rows serde_json::value::RawValue>> =
            Vec::with_capacity(hint_size.unwrap_or_default());

        for row in rows.into_iter() {
            let id = CibouletteIdBuilder::Text(Cow::Borrowed(row.id));
            let identifier =
                CibouletteResourceIdentifierBuilder::new(Some(id), Cow::Borrowed(row.type_));
            let related_identifier = match (row.related_type, row.related_id) {
                (Some(type_), Some(id)) => {
                    let related_id = CibouletteIdBuilder::Text(Cow::Borrowed(id));
                    let related_identifier = CibouletteResourceIdentifierBuilder::new(
                        Some(related_id),
                        Cow::Borrowed(type_),
                    );
                    Some(related_identifier)
                }
                _ => None,
            };
            res.push(CibouletteResponseElement::new(
                &store,
                main_type,
                identifier,
                row.data,
                related_identifier,
            )?);
        }
        Ok(res)
    }
}
