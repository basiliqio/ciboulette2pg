use super::*;
use crate::builder::tests::*;
use basiliq_db_test_proc_macro::basiliq_test;
use ciboulette_test_helper::ciboulette::*;
use ciboulette_test_helper::*;
// use run_migrations::*;
use serde_json::json;
use std::convert::TryFrom;
use url::Url;
use uuid::Uuid;
mod delete;
mod init_values;
mod inserts;
mod query_params;
mod response_elements;
#[allow(clippy::mutex_atomic)]
// mod run_migrations;
mod select;
mod test_sql;
mod update;

use test_sql::snapshot_table;

fn check_uuid<'store, 'b>(
    value: insta::internals::Content,
    _path: insta::internals::ContentPath<'store>,
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
    	    "[].related_id" => insta::dynamic_redaction(check_uuid),
    	    "[].data.article_id" => insta::dynamic_redaction(check_uuid),
    	    "[].data.people_id" => insta::dynamic_redaction(check_uuid)
    	});
	};
}

#[macro_export]
macro_rules! check_response_elements {
	($rows:ident) => {
		let value = serde_json::to_value($rows).unwrap();

    	insta::assert_json_snapshot!(value,
    	{
    	    "[].identifier.id" => insta::dynamic_redaction(check_uuid),
    	    "[].related.id" => insta::dynamic_redaction(check_uuid),
    	    "[].data.article_id" => insta::dynamic_redaction(check_uuid),
    	    "[].data.people_id" => insta::dynamic_redaction(check_uuid)
    	});
	};
}
