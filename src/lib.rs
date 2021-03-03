#![warn(clippy::all)]
mod builder;
mod errors;
pub mod graph_walker;
mod safe_ident;
mod table_store;
mod value;

pub use builder::Ciboulette2PostgresBuilder;
use ciboulette::*;
pub use errors::Ciboulette2SqlError;
use std::borrow::Cow;
use std::convert::TryFrom;
pub use table_store::{Ciboulette2PostgresTableSettings, Ciboulette2PostgresTableStore};
pub use value::Ciboulette2SqlValue;

use builder::Ciboulette2SqlArguments;
use getset::Getters;
use messy_json::{MessyJson, MessyJsonObject, MessyJsonObjectValue, MessyJsonValue};
use safe_ident::Ciboulette2PostgresSafeIdent;
use std::collections::BTreeMap;

const POSTGRES_QUOTE: &[u8] = b"\"";
