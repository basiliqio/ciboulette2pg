use super::*;

#[test]
fn simple() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let table = Ciboulette2PostgresTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("cte_main_insert"),
    );
    builder
        .gen_select(&table, vec![("first-name", None), ("last-name", None)])
        .unwrap();
    let res = builder.build().unwrap();

    insta::assert_debug_snapshot!(res);
}

#[test]
fn alias() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let table = Ciboulette2PostgresTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("cte_main_insert"),
    );
    builder
        .gen_select(
            &table,
            vec![("first-name", Some("fn")), ("last-name", Some("ln"))],
        )
        .unwrap();
    let res = builder.build().unwrap();

    insta::assert_debug_snapshot!(res);
}

#[test]
fn all() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let table = Ciboulette2PostgresTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("cte_main_insert"),
    );
    builder.gen_select(&table, vec![]).unwrap();
    let res = builder.build().unwrap();

    insta::assert_debug_snapshot!(res);
}
