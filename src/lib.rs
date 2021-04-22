#![warn(clippy::all)]
mod builder;
mod ciboulette_row;
mod errors;
mod response_type;
mod safe_ident;
mod table_store;
#[cfg(test)]
mod tests;

mod value;

use arcstr::ArcStr;
pub use builder::Ciboulette2PostgresBuilder;
pub use builder::Ciboulette2SqlArguments;
use ciboulette::*;
pub use ciboulette_row::Ciboulette2PostgresRow;
pub use errors::Ciboulette2SqlBufError;
pub use errors::Ciboulette2SqlError;
use getset::Getters;
use messy_json::{
    MessyJsonExpected, MessyJsonInner, MessyJsonNullType, MessyJsonObject, MessyJsonObjectValue,
    MessyJsonValue,
};
use response_type::Ciboulette2PostgresResponseType;
pub use safe_ident::Ciboulette2PostgresSafeIdent;
use safe_ident::Ciboulette2PostgresSafeIdentModifier;
use safe_ident::{
    CIBOULETTE_CTE_FINAL_MAIN_DATA, CIBOULETTE_DATA_IDENT, CIBOULETTE_DATA_SUFFIX,
    CIBOULETTE_EMPTY_IDENT, CIBOULETTE_ID_IDENT, CIBOULETTE_INSERT_SUFFIX,
    CIBOULETTE_MAIN_IDENTIFIER, CIBOULETTE_RELATED_ID_IDENT, CIBOULETTE_RELATED_TYPE_IDENT,
    CIBOULETTE_REL_PREFIX, CIBOULETTE_SORT_PREFIX, CIBOULETTE_TYPE_IDENT, CIBOULETTE_UPDATE_SUFFIX,
    TEXT_IDENT,
};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::sync::Arc;
pub use table_store::{
    Ciboulette2PostgresId, Ciboulette2PostgresTable, Ciboulette2PostgresTableStore,
};
pub use value::Ciboulette2SqlValue;
const POSTGRES_QUOTE: &[u8] = b"\"";
