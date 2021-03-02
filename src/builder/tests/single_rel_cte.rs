use super::*;

#[test]
fn simple() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let rel_table = Ciboulette2PostgresTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("favorite_color"),
    );
    let main_table = Ciboulette2PostgresTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("cte_peoples"),
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
            "favorite_color",
        )
        .unwrap();

    let res = builder.build().unwrap();
    insta::assert_debug_snapshot!(res);
}

#[test]
fn sparse() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let rel_table = Ciboulette2PostgresTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("favorite_color"),
    );
    let main_table = Ciboulette2PostgresTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("cte_peoples"),
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
            "favorite_color",
        )
        .unwrap();

    let res = builder.build().unwrap();
    insta::assert_debug_snapshot!(res);
}

#[test]
fn empty_sparse() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let rel_table = Ciboulette2PostgresTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("favorite_color"),
    );
    let main_table = Ciboulette2PostgresTableSettings::new(
        Cow::Borrowed("id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("cte_peoples"),
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
            "favorite_color",
        )
        .unwrap();

    let res = builder.build().unwrap();
    insta::assert_debug_snapshot!(res);
}
