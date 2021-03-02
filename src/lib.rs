#![warn(clippy::all)]
mod builder;
mod errors;
pub mod graph_walker;
mod table_store;
mod value;

pub use builder::Ciboulette2PostgresBuilder;
use ciboulette::*;
pub use errors::Ciboulette2SqlError;
use std::borrow::Cow;
pub use table_store::{Ciboulette2PostgresTableSettings, Ciboulette2PostgresTableStore};
pub use value::Ciboulette2SqlValue;

use builder::Ciboulette2SqlArguments;
use getset::Getters;
use messy_json::{MessyJson, MessyJsonObject, MessyJsonObjectValue, MessyJsonValue};
use std::collections::BTreeMap;

const POSTGRES_QUOTE: &[u8] = b"\"";
