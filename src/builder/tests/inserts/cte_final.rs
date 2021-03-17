use super::*;

#[test]
fn simple() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let store = gen_bag();
    let table_store = gen_table_store(&store);
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from("id").unwrap()),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap(),
    );
    let url = Url::parse("http://localhost/peoples").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
    let state = Ciboulette2PostgresBuilderState::new(
        &store,
        &table_store,
        ciboulette_request.path(),
        ciboulette_request.query(),
    )
    .unwrap();
    builder
        .gen_select_cte_final(
            &state,
            &dest_table,
            store.get_type("peoples").unwrap(),
            [].iter(),
            true,
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn sparse() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let store = gen_bag();
    let table_store = gen_table_store(&store);
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from("id").unwrap()),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap(),
    );
    let url = Url::parse("http://localhost/peoples?fields[peoples]=first-name").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
    let state = Ciboulette2PostgresBuilderState::new(
        &store,
        &table_store,
        ciboulette_request.path(),
        ciboulette_request.query(),
    )
    .unwrap();

    builder
        .gen_select_cte_final(
            &state,
            &dest_table,
            store.get_type("peoples").unwrap(),
            [].iter(),
            true,
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn empty_sparse() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let store = gen_bag();
    let table_store = gen_table_store(&store);
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from("id").unwrap()),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap(),
    );
    let url = Url::parse("http://localhost/peoples?fields[peoples]=").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
    let state = Ciboulette2PostgresBuilderState::new(
        &store,
        &table_store,
        ciboulette_request.path(),
        ciboulette_request.query(),
    )
    .unwrap();
    builder
        .gen_select_cte_final(
            &state,
            &dest_table,
            store.get_type("peoples").unwrap(),
            [].iter(),
            true,
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn non_included() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let store = gen_bag();
    let table_store = gen_table_store(&store);
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from("id").unwrap()),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap(),
    );
    let url = Url::parse("http://localhost/peoples").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
    let state = Ciboulette2PostgresBuilderState::new(
        &store,
        &table_store,
        ciboulette_request.path(),
        ciboulette_request.query(),
    )
    .unwrap();
    builder
        .gen_select_cte_final(
            &state,
            &dest_table,
            store.get_type("peoples").unwrap(),
            [].iter(),
            false,
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql!(res);
}
