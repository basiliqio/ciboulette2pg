use super::*;

#[test]
fn multi() {
    let store = gen_bag();
    let table_store = gen_table_store(&store);
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
        .gen_union_select_all(&table_store, &CibouletteSortingMap::default())
        .unwrap();
    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn single() {
    let store = gen_bag();
    let table_store = gen_table_store(&store);
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
        .gen_union_select_all(&table_store, &CibouletteSortingMap::default())
        .unwrap();
    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn no_table() {
    let store = gen_bag();
    let table_store = gen_table_store(&store);
    let mut builder = Ciboulette2PostgresBuilder::default();
    builder
        .gen_union_select_all(&table_store, &CibouletteSortingMap::default())
        .unwrap();
    let res = builder.build().unwrap();
    test_sql!(res);
}
