use super::*;

#[test]
fn simple() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("mysimpletable").unwrap(),
    );
    let params: Vec<(&str, Ciboulette2SqlValue<'_>)> = vec![
        (
            "first-name",
            Ciboulette2SqlValue::Text(Some(Cow::Borrowed("hello"))),
        ),
        (
            "last-name",
            Ciboulette2SqlValue::Text(Some(Cow::Borrowed("world"))),
        ),
    ];
    builder.gen_insert_normal(&table, params, true).unwrap();
    let res = builder.build().unwrap();

    insta::assert_debug_snapshot!(res);
}

#[test]
fn no_returning() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("mysimpletable").unwrap(),
    );
    let params: Vec<(&str, Ciboulette2SqlValue<'_>)> = vec![
        (
            "first-name",
            Ciboulette2SqlValue::Text(Some(Cow::Borrowed("hello"))),
        ),
        (
            "last-name",
            Ciboulette2SqlValue::Text(Some(Cow::Borrowed("world"))),
        ),
    ];
    builder.gen_insert_normal(&table, params, false).unwrap();
    let res = builder.build().unwrap();

    insta::assert_debug_snapshot!(res);
}

#[test]
fn no_params() {
    let mut builder = Ciboulette2PostgresBuilder::default();
    let table = Ciboulette2PostgresTableSettings::new(
        Ciboulette2PostgresSafeIdent::try_from("id").unwrap(),
        Ciboulette2PostgresSafeIdent::try_from("uuid").unwrap(),
        Some(Ciboulette2PostgresSafeIdent::try_from("public").unwrap()),
        Ciboulette2PostgresSafeIdent::try_from("mysimpletable").unwrap(),
    );
    builder.gen_insert_normal(&table, vec![], true).unwrap();
    let res = builder.build().unwrap();

    insta::assert_debug_snapshot!(res);
}
