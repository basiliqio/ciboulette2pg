#![warn(clippy::all)]
mod builder;
mod ciboulette_row;
mod errors;
pub mod graph_walker;
mod response;
mod safe_ident;
mod table_store;

#[cfg(test)]
mod tests;

mod value;

pub use builder::Ciboulette2PostgresBuilder;
use builder::Ciboulette2SqlArguments;
use ciboulette::*;
pub use ciboulette_row::Ciboulette2PostgresRow;
pub use errors::Ciboulette2SqlError;
use getset::Getters;
use messy_json::{
    MessyJson, MessyJsonNullType, MessyJsonObject, MessyJsonObjectValue, MessyJsonValue,
};
use response::response_type::Ciboulette2PostgresResponseType;
use safe_ident::Ciboulette2PostgresSafeIdent;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::convert::TryFrom;
pub use table_store::{
    Ciboulette2PostgresId, Ciboulette2PostgresTable, Ciboulette2PostgresTableStore,
};
pub use value::Ciboulette2SqlValue;

const POSTGRES_QUOTE: &[u8] = b"\"";
