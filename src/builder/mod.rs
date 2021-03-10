use super::*;
use getset::{Getters, MutGetters};
use numtoa::NumToA;
use std::io::Write;

mod delete;
mod insert;
mod select;
mod update;
mod utils;

#[cfg(test)]
mod tests;

use crate::graph_walker::main::Ciboulette2PostgresMain;
use crate::graph_walker::relationships::Ciboulette2PostgresRelationships;

type Ciboulette2PostgresBuf = buf_redux::BufWriter<std::io::Cursor<Vec<u8>>>;
type CibouletteSortingMap<'a> =
    HashMap<&'a CibouletteResourceType<'a>, Vec<&'a CibouletteSortingElement<'a>>>;

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
    included_tables:
        BTreeMap<&'a Ciboulette2PostgresTableSettings<'a>, Ciboulette2PostgresTableSettings<'a>>,
    nb_cte: usize,
}

impl<'a> Default for Ciboulette2PostgresBuilder<'a> {
    fn default() -> Self {
        Ciboulette2PostgresBuilder {
            buf: Ciboulette2PostgresBuf::new_ringbuf(std::io::Cursor::new(Vec::with_capacity(
                4096,
            ))),
            params: Ciboulette2SqlArguments::with_capacity(128),
            included_tables: BTreeMap::default(),
            nb_cte: 0,
        }
    }
}
