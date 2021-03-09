use super::*;

#[test]
fn simple() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("people_article").unwrap(),
    );

    let main_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("cte_main_insert").unwrap(),
    );
    let rel_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("cte_rel_data").unwrap(),
    );

    builder
        .gen_rel_insert(
            &dest_table,
            &Ciboulette2PostgresSafeIdent::try_from("author_id").unwrap(),
            &Ciboulette2PostgresSafeIdent::try_from("article_id").unwrap(),
            &main_table,
            &rel_table,
        )
        .unwrap();
    let res = builder.build().unwrap();

    test_sql(&res);
}
