use super::*;
use getset::{Getters, MutGetters};
use numtoa::NumToA;
use std::io::Write;

mod additional_fields;
mod builder_state;
mod delete;
mod extracting_data;
mod field_name;
mod insert;
mod is_needed;
mod relating_field;
// mod relationships;
mod select;
mod update;
mod utils;

use relating_field::Ciboulette2PostgresRelatingField;

#[macro_export]
macro_rules! get_state {
    ($ciboulette_store:expr, $ciboulette_table_store:expr, $req:expr) => {
        Ciboulette2PostgresBuilderState::new(
            $ciboulette_store,
            $ciboulette_table_store,
            $req.path(),
            $req.query(),
            Ciboulette2PostgresResponseType::from(*$req.expected_response_type()),
        )
    };
}

#[cfg(test)]
pub mod tests;

// use crate::graph_walker::main::Ciboulette2PostgresMainResourceInformations;
// use crate::graph_walker::relationships::{
//     Ciboulette2PostgresMainResourceRelationships, Ciboulette2PostgresMultiRelationships,
// };
use additional_fields::{Ciboulette2SqlAdditionalField, Ciboulette2SqlAdditionalFieldType};
use builder_state::Ciboulette2PostgresBuilderState;
use extracting_data::*;
use field_name::Ciboulette2PostgresTableField;
// use relationships::Ciboulette2SqlQueryRels;

type Ciboulette2PostgresBuf = buf_redux::BufWriter<std::io::Cursor<Vec<u8>>>;

#[derive(Clone, Debug, Default, Getters)]
#[getset(get = "pub")]
pub struct Ciboulette2SqlArguments<'request> {
    inner: Vec<Ciboulette2SqlValue<'request>>,
}

impl<'request> Ciboulette2SqlArguments<'request> {
    pub fn with_capacity(cap: usize) -> Self {
        Ciboulette2SqlArguments {
            inner: Vec::with_capacity(cap),
        }
    }

    pub fn take(self) -> Vec<Ciboulette2SqlValue<'request>> {
        self.inner
    }
}

impl<'request> std::ops::Deref for Ciboulette2SqlArguments<'request> {
    type Target = Vec<Ciboulette2SqlValue<'request>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'request> std::ops::DerefMut for Ciboulette2SqlArguments<'request> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresBuilder<'request> {
    buf: Ciboulette2PostgresBuf,
    params: Ciboulette2SqlArguments<'request>,
    #[getset(get_mut = "pub")]
    working_tables: BTreeMap<
        Ciboulette2PostgresSafeIdent,
        (Ciboulette2PostgresTable, Ciboulette2PostgresResponseType),
    >,
}

impl<'request> Default for Ciboulette2PostgresBuilder<'request> {
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

impl<'request> Ciboulette2PostgresBuilder<'request> {
    pub(crate) fn add_working_table(
        &mut self,
        main_table: &Ciboulette2PostgresTable,
        val: (Ciboulette2PostgresTable, Ciboulette2PostgresResponseType),
    ) {
        self.working_tables.insert(main_table.name().clone(), val);
    }
}
