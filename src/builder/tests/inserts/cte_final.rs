use super::*;

#[test]
fn simple() {
    let mut builder = Ciboulette2PgBuilder::default();
    let store = gen_bag();
    let table_store = gen_table_store(&store);
    let dest_table = Ciboulette2PgTable::new(
        vec![Ciboulette2PgId::Uuid(
            Ciboulette2PgSafeIdent::try_from("id").unwrap(),
        )],
        Some(Ciboulette2PgSafeIdent::try_from("public").unwrap()),
        Ciboulette2PgSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap().clone(),
    );
    let url = Url::parse("http://localhost/peoples").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
    let state = Ciboulette2PgBuilderState::new(
        &store,
        &table_store,
        ciboulette_request.path(),
        ciboulette_request.query(),
        Ciboulette2PgResponseType::from(*ciboulette_request.expected_response_type()),
    )
    .unwrap();
    builder
        .gen_select_cte(
            &state,
            &dest_table,
            store.get_type("peoples").unwrap().clone(),
            None,
            [].iter(),
            true,
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn sparse() {
    let mut builder = Ciboulette2PgBuilder::default();
    let store = gen_bag();
    let table_store = gen_table_store(&store);
    let dest_table = Ciboulette2PgTable::new(
        vec![Ciboulette2PgId::Uuid(
            Ciboulette2PgSafeIdent::try_from("id").unwrap(),
        )],
        Some(Ciboulette2PgSafeIdent::try_from("public").unwrap()),
        Ciboulette2PgSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap().clone(),
    );
    let url = Url::parse("http://localhost/peoples?fields[peoples]=first-name").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
    let state = Ciboulette2PgBuilderState::new(
        &store,
        &table_store,
        ciboulette_request.path(),
        ciboulette_request.query(),
        Ciboulette2PgResponseType::from(*ciboulette_request.expected_response_type()),
    )
    .unwrap();

    builder
        .gen_select_cte(
            &state,
            &dest_table,
            store.get_type("peoples").unwrap().clone(),
            None,
            [].iter(),
            true,
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn empty_sparse() {
    let mut builder = Ciboulette2PgBuilder::default();
    let store = gen_bag();
    let table_store = gen_table_store(&store);
    let dest_table = Ciboulette2PgTable::new(
        vec![Ciboulette2PgId::Uuid(
            Ciboulette2PgSafeIdent::try_from("id").unwrap(),
        )],
        Some(Ciboulette2PgSafeIdent::try_from("public").unwrap()),
        Ciboulette2PgSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap().clone(),
    );
    let url = Url::parse("http://localhost/peoples?fields[peoples]=").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
    let state = Ciboulette2PgBuilderState::new(
        &store,
        &table_store,
        ciboulette_request.path(),
        ciboulette_request.query(),
        Ciboulette2PgResponseType::from(*ciboulette_request.expected_response_type()),
    )
    .unwrap();
    builder
        .gen_select_cte(
            &state,
            &dest_table,
            store.get_type("peoples").unwrap().clone(),
            None,
            [].iter(),
            true,
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn non_included() {
    let mut builder = Ciboulette2PgBuilder::default();
    let store = gen_bag();
    let table_store = gen_table_store(&store);
    let dest_table = Ciboulette2PgTable::new(
        vec![Ciboulette2PgId::Uuid(
            Ciboulette2PgSafeIdent::try_from("id").unwrap(),
        )],
        Some(Ciboulette2PgSafeIdent::try_from("public").unwrap()),
        Ciboulette2PgSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap().clone(),
    );
    let url = Url::parse("http://localhost/peoples").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
    let state = Ciboulette2PgBuilderState::new(
        &store,
        &table_store,
        ciboulette_request.path(),
        ciboulette_request.query(),
        Ciboulette2PgResponseType::from(*ciboulette_request.expected_response_type()),
    )
    .unwrap();
    builder
        .gen_select_cte(
            &state,
            &dest_table,
            store.get_type("peoples").unwrap().clone(),
            None,
            [].iter(),
            false,
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql!(res);
}
