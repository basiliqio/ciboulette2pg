use super::*;
use std::convert::TryFrom;
use url::Url;

fn gen_req<'a>(store: &'a CibouletteStore<'a>, parsed_url: &'a Url) -> CibouletteCreateRequest<'a> {
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "peoples",
			"attributes":
			{
				"first-name": "Hello",
				"last-name": "World"
			}
		}
	}
	"#,
    );

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&store).unwrap();
    let ciboulette_request = CibouletteCreateRequest::try_from(request).unwrap();
    ciboulette_request
}

#[test]
fn no_sparse_fields() {
    let mut builder = Ciboulette2PostgresBuilder::new();
    let dest_table = CibouletteTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("peoples"),
    );
    let store = gen_bag();
    let url = Url::parse("http://localhost/peoples").unwrap();
    let ciboulette_request = gen_req(&store, &url);
    builder
        .gen_json_builder(
            &dest_table,
            store.get_type("peoples").unwrap(),
            ciboulette_request.query(),
        )
        .unwrap();

    let res = builder.build().unwrap();
    insta::assert_debug_snapshot!(res);
}

#[test]
fn sparse_field() {
    let mut builder = Ciboulette2PostgresBuilder::new();
    let dest_table = CibouletteTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("peoples"),
    );
    let store = gen_bag();
    let url = Url::parse("http://localhost/peoples?fields[peoples]=first-name,last-name").unwrap();
    let ciboulette_request = gen_req(&store, &url);
    builder
        .gen_json_builder(
            &dest_table,
            store.get_type("peoples").unwrap(),
            ciboulette_request.query(),
        )
        .unwrap();

    let res = builder.build().unwrap();
    insta::assert_debug_snapshot!(res);
}

#[test]
fn sparse_field_empty() {
    let mut builder = Ciboulette2PostgresBuilder::new();
    let dest_table = CibouletteTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("peoples"),
    );
    let store = gen_bag();
    let url = Url::parse("http://localhost/peoples?fields[peoples]=").unwrap();
    let ciboulette_request = gen_req(&store, &url);
    builder
        .gen_json_builder(
            &dest_table,
            store.get_type("peoples").unwrap(),
            ciboulette_request.query(),
        )
        .unwrap();

    let res = builder.build().unwrap();
    insta::assert_debug_snapshot!(res);
}
