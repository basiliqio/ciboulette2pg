use super::*;

#[test]
fn simple() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let store = gen_bag();
    let table = Ciboulette2PostgresTable::new(
        Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from("id").unwrap()),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("mysimpletable").unwrap(),
        store.get_type("peoples").unwrap().clone(),
    );
    let params: Vec<(ArcStr, Ciboulette2SqlValue<'_>)> = vec![
        (
            ArcStr::from("first-name"),
            Ciboulette2SqlValue::Text(Some(Cow::Borrowed("hello"))),
        ),
        (
            ArcStr::from("last-name"),
            Ciboulette2SqlValue::Text(Some(Cow::Borrowed("world"))),
        ),
    ];
    builder.gen_insert_normal(&table, params, true).unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn no_returning() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let store = gen_bag();
    let table = Ciboulette2PostgresTable::new(
        Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from("id").unwrap()),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("mysimpletable").unwrap(),
        store.get_type("peoples").unwrap().clone(),
    );
    let params: Vec<(ArcStr, Ciboulette2SqlValue<'_>)> = vec![
        (
            ArcStr::from("first-name"),
            Ciboulette2SqlValue::Text(Some(Cow::Borrowed("hello"))),
        ),
        (
            ArcStr::from("last-name"),
            Ciboulette2SqlValue::Text(Some(Cow::Borrowed("world"))),
        ),
    ];
    builder.gen_insert_normal(&table, params, false).unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn no_params() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let store = gen_bag();
    let table = Ciboulette2PostgresTable::new(
        Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from("id").unwrap()),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("mysimpletable").unwrap(),
        store.get_type("peoples").unwrap().clone(),
    );
    builder.gen_insert_normal(&table, vec![], true).unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}
