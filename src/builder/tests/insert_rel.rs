use super::*;

#[test]
fn simple() {
    let mut builder = Ciboulette2PostgresBuilder::new();
    let dest_table = CibouletteTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("people_article"),
    );

    let main_table = CibouletteTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("cte_main_insert"),
    );
    let rel_table = CibouletteTableSettings::new(
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
