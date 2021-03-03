use super::*;

#[test]
fn no_sparse_fields() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
    );
    let store = gen_bag();
    let url = Url::parse("http://localhost/peoples").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
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
    let mut builder = Ciboulette2PostgresBuilder::default();
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
    );
    let store = gen_bag();
    let url = Url::parse("http://localhost/peoples?fields[peoples]=first-name,last-name").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
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
    let mut builder = Ciboulette2PostgresBuilder::default();
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
    );
    let store = gen_bag();
    let url = Url::parse("http://localhost/peoples?fields[peoples]=").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
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
