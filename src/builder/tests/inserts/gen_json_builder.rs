use super::*;

#[test]
fn no_sparse_fields() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let store = gen_bag();
    let dest_table = Ciboulette2PostgresTable::new(
        Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from("id").unwrap()),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap().clone(),
    );
    let url = Url::parse("http://localhost/peoples").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
    builder
        .gen_json_builder(
            &dest_table,
            store.get_type("peoples").unwrap().clone(),
            ciboulette_request.query(),
            true,
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn sparse_field() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let store = gen_bag();
    let dest_table = Ciboulette2PostgresTable::new(
        Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from("id").unwrap()),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap().clone(),
    );
    let url = Url::parse("http://localhost/peoples?fields[peoples]=first-name,last-name").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
    builder
        .gen_json_builder(
            &dest_table,
            store.get_type("peoples").unwrap().clone(),
            ciboulette_request.query(),
            true,
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn sparse_field_empty() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let store = gen_bag();
    let dest_table = Ciboulette2PostgresTable::new(
        Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from("id").unwrap()),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap().clone(),
    );
    let url = Url::parse("http://localhost/peoples?fields[peoples]=").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
    builder
        .gen_json_builder(
            &dest_table,
            store.get_type("peoples").unwrap().clone(),
            ciboulette_request.query(),
            true,
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn not_included() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let store = gen_bag();
    let dest_table = Ciboulette2PostgresTable::new(
        Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from("id").unwrap()),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap().clone(),
    );
    let url = Url::parse("http://localhost/peoples").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
    builder
        .gen_json_builder(
            &dest_table,
            store.get_type("peoples").unwrap().clone(),
            ciboulette_request.query(),
            false,
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql!(res);
}
