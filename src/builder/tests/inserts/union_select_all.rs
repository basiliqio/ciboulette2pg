use super::*;

#[test]
fn multi() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
    );

    let from_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("articles").unwrap(),
    );
    builder
        .included_tables
        .insert(&from_table, from_table.clone());
    builder
        .included_tables
        .insert(&dest_table, dest_table.clone());
    builder.gen_union_select_all().unwrap();
    let res = builder.build().unwrap();
    insta::assert_debug_snapshot!(res);
}

#[test]
fn single() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
    );
    builder
        .included_tables
        .insert(&dest_table, dest_table.clone());
    builder.gen_union_select_all().unwrap();
    let res = builder.build().unwrap();
    insta::assert_debug_snapshot!(res);
}

#[test]
fn no_table() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    builder.gen_union_select_all().unwrap();
    let res = builder.build().unwrap();
    insta::assert_debug_snapshot!(res);
}
