use super::*;

fn gen_req<'a>(store: &'a CibouletteStore<'a>, q: &'a Url) -> CibouletteQueryParameters<'a> {
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;
    const BODY: Option<&str> = None;

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &q, &BODY);
    let CibouletteRequest { query, .. } = req_builder.build(&store).unwrap();
    query.unwrap_or_default()
}

#[test]
fn multi() {
    let store = gen_bag();
    let q_url = Url::parse("http://localhost/peoples").unwrap();
    let q = gen_req(&store, &q_url);
    let mut builder = Ciboulette2PostgresBuilder::default();
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap(),
    );

    let from_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("articles").unwrap(),
        store.get_type("articles").unwrap(),
    );
    builder
        .included_tables
        .insert(&from_table, from_table.clone());
    builder
        .included_tables
        .insert(&dest_table, dest_table.clone());
    builder.gen_union_select_all(&q).unwrap();
    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn single() {
    let store = gen_bag();
    let q_url = Url::parse("http://localhost/peoples").unwrap();
    let q = gen_req(&store, &q_url);
    let mut builder = Ciboulette2PostgresBuilder::default();
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap(),
    );
    builder
        .included_tables
        .insert(&dest_table, dest_table.clone());
    builder.gen_union_select_all(&q).unwrap();
    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn no_table() {
    let store = gen_bag();
    let q_url = Url::parse("http://localhost/peoples").unwrap();
    let q = gen_req(&store, &q_url);
    let mut builder = Ciboulette2PostgresBuilder::default();
    builder.gen_union_select_all(&q).unwrap();
    let res = builder.build().unwrap();
    test_sql!(res);
}
