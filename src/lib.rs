mod builder;
pub mod creation;
mod errors;
mod request_type;
mod utils;
mod value;

pub use builder::Ciboulette2PostgresBuilder;
use ciboulette::*;
pub use errors::Ciboulette2SqlError;
use std::borrow::Cow;
pub use value::Ciboulette2SqlValue;

use builder::Ciboulette2SqlArguments;
use getset::Getters;
use messy_json::{MessyJson, MessyJsonObjectValue, MessyJsonValue};
use std::collections::BTreeMap;

const POSTGRES_QUOTE: &[u8] = b"\"";
const POSTGRES_SPACE: &[u8] = b" ";

#[derive(Getters, Clone, Debug, Default)]
#[getset(get = "pub")]
pub struct CibouletteTableSettings<'a> {
    id_name: Cow<'a, str>,
    id_type: Cow<'a, str>,
    schema: Option<Cow<'a, str>>,
    name: Cow<'a, str>,
}

impl<'a> CibouletteTableSettings<'a> {
    pub fn new(
        id_name: Cow<'a, str>,
        id_type: Cow<'a, str>,
        schema: Option<Cow<'a, str>>,
        name: Cow<'a, str>,
    ) -> Self {
        CibouletteTableSettings {
            id_name,
            id_type,
            schema,
            name,
        }
    }
}
