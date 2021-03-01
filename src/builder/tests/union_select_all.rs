use super::*;

#[test]
fn multi() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("peoples"),
    );

    let from_table = Ciboulette2PostgresTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("articles"),
    );
    builder
        .gen_union_select_all(vec![from_table, dest_table].iter())
        .unwrap();
    let res = builder.build().unwrap();
    insta::assert_debug_snapshot!(res);
}

#[test]
fn single() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("peoples"),
    );
    builder
        .gen_union_select_all(vec![dest_table].iter())
        .unwrap();
    let res = builder.build().unwrap();
    insta::assert_debug_snapshot!(res);
}

#[test]
fn no_table() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    builder.gen_union_select_all(vec![].iter()).unwrap();
    let res = builder.build().unwrap();
    insta::assert_debug_snapshot!(res);
}
