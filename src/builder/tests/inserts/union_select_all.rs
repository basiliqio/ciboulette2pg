use std::convert::TryInto;

use super::*;

lazy_static::lazy_static! {
    /// This is an example for using doc comment attributes
    static ref PARSED_URL: Url = {
        Url::parse("http://localhost/peoples").unwrap()
    };
}

fn gen_request<'a>(store: &'a CibouletteStore<'a>) -> CibouletteReadRequest<'_> {
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &PARSED_URL, &None);
    let request = req_builder.build(&store).unwrap();
    request.try_into().unwrap()
}

#[test]
fn multi() {
    let store = gen_bag();
    let table_store = gen_table_store(&store);
    let req = gen_request(&store);
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
    builder
        .gen_union_select_all(&store, &table_store, &req.query(), &dest_table)
        .unwrap();
    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn single() {
    let store = gen_bag();
    let table_store = gen_table_store(&store);
    let req = gen_request(&store);
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
    builder
        .gen_union_select_all(&store, &table_store, &req.query(), &dest_table)
        .unwrap();
    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn no_table() {
    let store = gen_bag();
    let table_store = gen_table_store(&store);
    let req = gen_request(&store);
    let mut builder = Ciboulette2PostgresBuilder::default();
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap(),
    );
    builder
        .gen_union_select_all(&store, &table_store, &req.query(), &dest_table)
        .unwrap();
    let res = builder.build().unwrap();
    test_sql!(res);
}
