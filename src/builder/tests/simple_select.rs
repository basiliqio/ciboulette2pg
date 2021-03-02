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
        .gen_select(
            &table,
            vec![("first-name", None, None), ("last-name", None, None)],
        )
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
            vec![
                ("first-name", Some("fn"), None),
                ("last-name", Some("ln"), None),
            ],
        )
        .unwrap();
    let res = builder.build().unwrap();

    insta::assert_debug_snapshot!(res);
}

#[test]
fn cast() {
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
            vec![
                ("first-name", Some("fn"), Some("text")),
                ("last-name", Some("ln"), Some("other_text_type")),
            ],
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
