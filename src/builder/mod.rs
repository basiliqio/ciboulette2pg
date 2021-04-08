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
mod relating_field;
mod relationships;
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

    static ref CIBOULETTE_RELATED_ID_IDENT: Ciboulette2PostgresSafeIdent<'static> = {
        Ciboulette2PostgresSafeIdent::try_from("related_id").unwrap()
    };

    static ref CIBOULETTE_RELATED_TYPE_IDENT: Ciboulette2PostgresSafeIdent<'static> = {
        Ciboulette2PostgresSafeIdent::try_from("related_type").unwrap()
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
pub struct Ciboulette2PostgresBuilder<'store, 'request> {
    buf: Ciboulette2PostgresBuf,
    params: Ciboulette2SqlArguments<'request>,
    #[getset(get_mut = "pub")]
    working_tables: BTreeMap<
        &'store Ciboulette2PostgresTable<'store>,
        (
            Ciboulette2PostgresTable<'store>,
            Ciboulette2PostgresResponseType,
        ),
    >,
}

impl<'store, 'request> Default for Ciboulette2PostgresBuilder<'store, 'request>
where
    'store: 'request,
{
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

impl<'store, 'request> Ciboulette2PostgresBuilder<'store, 'request>
where
    'store: 'request,
{
    pub(crate) fn add_working_table(
        &mut self,
        main_table: &'store Ciboulette2PostgresTable<'store>,
        val: (
            Ciboulette2PostgresTable<'store>,
            Ciboulette2PostgresResponseType,
        ),
    ) {
        self.working_tables.insert(main_table, val);
    }
}
