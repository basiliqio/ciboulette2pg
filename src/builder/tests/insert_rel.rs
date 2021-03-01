use super::*;

#[test]
fn simple() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("people_article"),
    );

    let main_table = Ciboulette2PostgresTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("cte_main_insert"),
    );
    let rel_table = Ciboulette2PostgresTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("cte_rel_data"),
    );

    builder
        .gen_rel_insert(
            &dest_table,
            "author_id",
            "article_id",
            &main_table,
            &rel_table,
        )
        .unwrap();
    let res = builder.build().unwrap();

    insta::assert_debug_snapshot!(res);
}
