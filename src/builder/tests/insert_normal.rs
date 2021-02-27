use super::*;

#[test]
fn simple() {
    let mut builder = Ciboulette2PostgresBuilder::new();
    let table = CibouletteTableSettings::new(
        "id".to_string(),
        "uuid".to_string(),
        "public".to_string(),
        "mysimpletable".to_string(),
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
    let mut builder = Ciboulette2PostgresBuilder::new();
    let table = CibouletteTableSettings::new(
        "id".to_string(),
        "uuid".to_string(),
        "public".to_string(),
        "mysimpletable".to_string(),
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
    let mut builder = Ciboulette2PostgresBuilder::new();
    let table = CibouletteTableSettings::new(
        "id".to_string(),
        "uuid".to_string(),
        "public".to_string(),
        "mysimpletable".to_string(),
    );
    builder.gen_insert_normal(&table, vec![], true).unwrap();
    let res = builder.build().unwrap();

    insta::assert_debug_snapshot!(res);
}
