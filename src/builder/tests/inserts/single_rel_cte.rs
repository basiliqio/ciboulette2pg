use super::*;

#[test]
fn simple() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let rel_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("favorite_color").unwrap(),
    );
    let main_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("cte_peoples").unwrap(),
    );
    let store = gen_bag();
    let url = Url::parse("http://localhost/peoples").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
    builder
        .gen_select_cte_single_rel(
            &rel_table,
            store.get_type("favorite_color").unwrap(),
            ciboulette_request.query(),
            &main_table,
            &Ciboulette2PostgresSafeIdent::try_from("favorite_color").unwrap(),
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql(&res);
}

#[test]
fn sparse() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let rel_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("favorite_color").unwrap(),
    );
    let main_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("cte_peoples").unwrap(),
    );
    let store = gen_bag();
    let url = Url::parse("http://localhost/peoples?fields[favorite_color]=color").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
    builder
        .gen_select_cte_single_rel(
            &rel_table,
            store.get_type("favorite_color").unwrap(),
            ciboulette_request.query(),
            &main_table,
            &Ciboulette2PostgresSafeIdent::try_from("favorite_color").unwrap(),
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql(&res);
}

#[test]
fn empty_sparse() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let rel_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("favorite_color").unwrap(),
    );
    let main_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("cte_peoples").unwrap(),
    );
    let store = gen_bag();
    let url = Url::parse("http://localhost/peoples?fields[favorite_color]=").unwrap();
    let ciboulette_request = gen_req_create_people(&store, &url);
    builder
        .gen_select_cte_single_rel(
            &rel_table,
            store.get_type("favorite_color").unwrap(),
            ciboulette_request.query(),
            &main_table,
            &Ciboulette2PostgresSafeIdent::try_from("favorite_color").unwrap(),
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql(&res);
}
