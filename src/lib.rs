mod builder;
pub mod creation;
mod errors;
mod request_type;
mod utils;
mod value;

pub use builder::Ciboulette2PostgresBuilder;
use ciboulette::*;
pub use errors::Ciboulette2SqlError;
pub use value::Ciboulette2SqlValue;

use builder::Ciboulette2SqlArguments;
use getset::Getters;
use messy_json::{MessyJson, MessyJsonObjectValue, MessyJsonValue};
use std::borrow::Cow;
use std::collections::BTreeMap;

const POSTGRES_QUOTE: &[u8] = b"\"";
const POSTGRES_SPACE: &[u8] = b" ";

#[derive(Getters)]
pub struct CibouletteTableSettings {
    id_name: String,
    id_type: String,
    schema: String,
    name: String,
}

impl CibouletteTableSettings {
    pub fn new(id_name: String, id_type: String, schema: String, name: String) -> Self {
        CibouletteTableSettings {
            id_name,
            id_type,
            schema,
            name,
        }
    }
}
