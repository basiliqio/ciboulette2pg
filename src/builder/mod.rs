use super::*;
use getset::{Getters, MutGetters};
use numtoa::NumToA;
use std::io::Write;

mod additional_fields;
mod builder_state;
mod delete;
mod field_name;
mod insert;
mod is_needed;
mod relationships;
mod select;
mod update;
mod utils;

#[macro_export]
macro_rules! get_state {
    ($ciboulette_store:expr, $ciboulette_table_store:expr, $req:expr) => {
        Ciboulette2PostgresBuilderState::new(
            $ciboulette_store,
            $ciboulette_table_store,
            $req.path(),
            $req.query(),
            $req.expected_response_type(),
        )
    };
}

lazy_static::lazy_static! {
    static ref CIBOULETTE_ID_IDENT: Ciboulette2PostgresSafeIdent<'static> = {
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap()
    };

    static ref CIBOULETTE_RAW_ID_IDENT: Ciboulette2PostgresSafeIdent<'static> = {
        Ciboulette2PostgresSafeIdent::try_from("_id").unwrap()
    };

    static ref CIBOULETTE_TYPE_IDENT: Ciboulette2PostgresSafeIdent<'static> = {
        Ciboulette2PostgresSafeIdent::try_from("type").unwrap()
    };

    static ref CIBOULETTE_DATA_IDENT: Ciboulette2PostgresSafeIdent<'static> = {
        Ciboulette2PostgresSafeIdent::try_from("data").unwrap()
    };
}

#[cfg(test)]
pub mod tests;

use crate::graph_walker::main::Ciboulette2PostgresMainResourceInformations;
use crate::graph_walker::relationships::{
    Ciboulette2PostgresMainResourceRelationships, Ciboulette2PostgresMultiRelationships,
};
use additional_fields::{Ciboulette2SqlAdditionalField, Ciboulette2SqlAdditionalFieldType};
use builder_state::Ciboulette2PostgresBuilderState;
use field_name::Ciboulette2PostgresTableField;
use relationships::Ciboulette2SqlQueryRels;

type Ciboulette2PostgresBuf = buf_redux::BufWriter<std::io::Cursor<Vec<u8>>>;

#[derive(Clone, Debug, Default, Getters)]
#[getset(get = "pub")]
pub struct Ciboulette2SqlArguments<'a> {
    inner: Vec<Ciboulette2SqlValue<'a>>,
}

impl<'a> Ciboulette2SqlArguments<'a> {
    pub fn with_capacity(cap: usize) -> Self {
        Ciboulette2SqlArguments {
            inner: Vec::with_capacity(cap),
        }
    }

    pub fn take(self) -> Vec<Ciboulette2SqlValue<'a>> {
        self.inner
    }
}

impl<'a> std::ops::Deref for Ciboulette2SqlArguments<'a> {
    type Target = Vec<Ciboulette2SqlValue<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> std::ops::DerefMut for Ciboulette2SqlArguments<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresBuilder<'a> {
    buf: Ciboulette2PostgresBuf,
    params: Ciboulette2SqlArguments<'a>,
    #[getset(get_mut = "pub")]
    working_tables: BTreeMap<
        &'a Ciboulette2PostgresTable<'a>,
        (Ciboulette2PostgresTable<'a>, CibouletteResponseRequiredType),
    >,
}

impl<'a> Default for Ciboulette2PostgresBuilder<'a> {
    fn default() -> Self {
        Ciboulette2PostgresBuilder {
            buf: Ciboulette2PostgresBuf::new_ringbuf(std::io::Cursor::new(Vec::with_capacity(
                4096,
            ))),
            params: Ciboulette2SqlArguments::with_capacity(128),
            working_tables: BTreeMap::default(),
        }
    }
}

impl<'a> Ciboulette2PostgresBuilder<'a> {
    pub(crate) fn add_working_table(
        &mut self,
        main_table: &'a Ciboulette2PostgresTable<'a>,
        val: (Ciboulette2PostgresTable<'a>, CibouletteResponseRequiredType),
    ) {
        self.working_tables.insert(main_table, val);
    }
}
