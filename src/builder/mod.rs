#![macro_use]
use super::*;
use getset::{Getters, MutGetters};
use numtoa::NumToA;
use std::io::Write;

macro_rules! get_state {
    ($ciboulette_store:expr, $ciboulette_table_store:expr, $req:expr) => {
        Ciboulette2PgBuilderState::new(
            $ciboulette_store,
            $ciboulette_table_store,
            $req.path(),
            $req.query(),
            Ciboulette2PgResponseType::from(*$req.expected_response_type()),
        )
    };
}

mod additional_fields;
mod builder_state;
mod delete;
mod extracting_data;
mod field_name;
mod insert;
mod relating_field;
mod select;
mod update;
mod utils;

use relating_field::Ciboulette2PgRelatingField;

#[cfg(test)]
pub mod tests;

use additional_fields::{Ciboulette2PgAdditionalField, Ciboulette2PgAdditionalFieldType};
use builder_state::Ciboulette2PgBuilderState;
use extracting_data::*;
use field_name::Ciboulette2PgTableField;

type Ciboulette2PgBuf = buf_redux::BufWriter<std::io::Cursor<Vec<u8>>>;

/// A list of parameters to send along side the query to database
#[derive(Clone, Debug, Default, Getters)]
#[getset(get = "pub")]
pub struct Ciboulette2PgArguments<'request> {
    inner: Vec<Ciboulette2PgValue<'request>>,
}

impl<'request> Ciboulette2PgArguments<'request> {
    pub fn with_capacity(cap: usize) -> Self {
        Ciboulette2PgArguments {
            inner: Vec::with_capacity(cap),
        }
    }

    pub fn take(self) -> Vec<Ciboulette2PgValue<'request>> {
        self.inner
    }
}

impl<'request> std::ops::Deref for Ciboulette2PgArguments<'request> {
    type Target = Vec<Ciboulette2PgValue<'request>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'request> std::ops::DerefMut for Ciboulette2PgArguments<'request> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// Ciboulette to Postgres Query builder
#[derive(Debug, Getters, MutGetters)]
#[getset(get = "pub")]
pub struct Ciboulette2PgBuilder<'request> {
    buf: Ciboulette2PgBuf,
    params: Ciboulette2PgArguments<'request>,
    #[getset(get_mut = "pub")]
    working_tables: BTreeMap<
        Vec<CibouletteResourceRelationshipDetails>,
        (Ciboulette2PgTable, Ciboulette2PgResponseType),
    >,
    cte_index: usize,
}

impl<'request> Default for Ciboulette2PgBuilder<'request> {
    fn default() -> Self {
        Ciboulette2PgBuilder {
            buf: Ciboulette2PgBuf::new_ringbuf(std::io::Cursor::new(Vec::with_capacity(4096))),
            params: Ciboulette2PgArguments::with_capacity(128),
            working_tables: BTreeMap::default(),
            cte_index: 0,
        }
    }
}

impl<'request> Ciboulette2PgBuilder<'request> {
    pub(crate) fn add_working_table(
        &mut self,
        rel_chain: Vec<CibouletteResourceRelationshipDetails>,
        table: Ciboulette2PgTable,
        response_type: Ciboulette2PgResponseType,
    ) -> Option<(Ciboulette2PgTable, Ciboulette2PgResponseType)> {
        self.working_tables
            .insert(rel_chain, (table, response_type))
    }

    pub(crate) fn get_new_cte_index(&mut self) -> usize {
        let res = self.cte_index;
        self.cte_index += 1;
        res
    }
}
