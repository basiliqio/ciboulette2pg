use super::*;
use crate::builder::tests::*;
use ciboulette2postgres_test_proc_macro::ciboulette2postgres_test;
use ciboulette_test_helper::ciboulette::*;
use ciboulette_test_helper::*;
use run_migrations::*;
use serde_json::json;
use std::convert::TryFrom;
use url::Url;
use uuid::Uuid;
mod delete;
mod init_values;
mod inserts;
mod query_params;
#[allow(clippy::mutex_atomic)]
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

#[macro_export]
macro_rules! check_rows {
	($rows:ident) => {
		let value = serde_json::to_value($rows).unwrap();

    	insta::assert_json_snapshot!(value,
    	{
    	    "[].id" => insta::dynamic_redaction(check_uuid),
    	    "[].data.article_id" => insta::dynamic_redaction(check_uuid),
    	    "[].data.people_id" => insta::dynamic_redaction(check_uuid)
    	});
	};
}
