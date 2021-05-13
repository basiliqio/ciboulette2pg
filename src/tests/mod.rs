use super::*;
use crate::builder::tests::*;
use basiliq_db_test_proc_macro::basiliq_test;
use ciboulette_test_helper::ciboulette::*;
use ciboulette_test_helper::*;
// use run_migrations::*;
use serde_json::json;
use std::convert::TryFrom;
use url::Url;
mod delete;
mod inserts;
mod query_params;
mod response_elements;
#[allow(clippy::mutex_atomic)]
// mod run_migrations;
mod select;
mod test_sql;
mod update;

use test_sql::snapshot_table;

fn check_uuid_list<'store, 'b>(
    value: insta::internals::Content,
    _path: insta::internals::ContentPath<'store>,
) -> &'b str {
    for parts in value.as_str().unwrap().split(",") {
        assert_eq!(parts.chars().filter(|&c| c == '-').count(), 4);
    }
    "[uuid]"
}

#[macro_export]
macro_rules! check_rows {
	($rows:ident) => {
		let value = serde_json::to_value($rows).unwrap();

    	insta::assert_json_snapshot!(value,
    	{
    	    "[].id" => insta::dynamic_redaction(check_uuid_list),
    	    "[].related_id" => insta::dynamic_redaction(check_uuid_list),
    	    "[].data.article_id" => insta::dynamic_redaction(check_uuid_list),
    	    "[].data.people_id" => insta::dynamic_redaction(check_uuid_list)
    	});
	};
}

#[macro_export]
macro_rules! check_response_elements {
	($rows:ident) => {
		let value = serde_json::to_value($rows).unwrap();

    	insta::assert_json_snapshot!(value,
    	{
    	    "[].identifier.id" => insta::dynamic_redaction(check_uuid_list),
    	    "[].related.id" => insta::dynamic_redaction(check_uuid_list),
    	    "[].data.article_id" => insta::dynamic_redaction(check_uuid_list),
    	    "[].data.people_id" => insta::dynamic_redaction(check_uuid_list)
    	});
	};
}
