use super::*;
use crate::builder::tests::*;
use ciboulette2postgres_test_proc_macro::ciboulette2postgres_test;
use ciboulette_test_helper::ciboulette::*;
use ciboulette_test_helper::*;
use run_migrations::*;
use serde_json::json;
use std::convert::TryFrom;
use url::Url;
mod delete;
mod init_values;
mod inserts;
mod run_migrations;
mod select;
mod test_sql;
mod update;

use test_sql::snapshot_table;

fn check_uuid<'a, 'b>(
    value: insta::internals::Content,
    _path: insta::internals::ContentPath<'a>,
) -> &'b str {
    assert_eq!(
        value
            .as_str()
            .unwrap()
            .chars()
            .filter(|&c| c == '-')
            .count(),
        4
    );
    "[uuid]"
}
