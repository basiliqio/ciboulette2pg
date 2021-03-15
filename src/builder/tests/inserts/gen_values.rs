use super::*;

#[test]
fn simple() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let store = gen_bag();
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from("id").unwrap()),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap(),
    );
    builder
        .gen_rel_values(
            vec![
                Ciboulette2SqlValue::Text(Some(Cow::Borrowed(
                    "e1ba7ab3-12f9-4a70-aced-a1637b6a3c23",
                ))),
                Ciboulette2SqlValue::Text(Some(Cow::Borrowed(
                    "4ba2994f-0282-4251-8061-2f9cb92808e6",
                ))),
            ],
            &dest_table,
            &Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from("id").unwrap()),
        )
        .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn empty() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let store = gen_bag();
    let dest_table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from("id").unwrap()),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("peoples").unwrap(),
        store.get_type("peoples").unwrap(),
    );
    let err = builder
        .gen_rel_values(
            vec![],
            &dest_table,
            &Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from("id").unwrap()),
        )
        .unwrap_err();

    assert_eq!(
        matches!(err, Ciboulette2SqlError::EmptyRelValue(x) if x == "id"),
        true
    );
}
