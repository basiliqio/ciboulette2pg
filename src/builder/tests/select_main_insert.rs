use super::*;

#[test]
fn simple() {
    let mut builder = Ciboulette2PostgresBuilder::new();
    let table = CibouletteTableSettings::new(
        Cow::Borrowed("my_id"),
        Cow::Borrowed("uuid"),
        Some(Cow::Borrowed("public")),
        Cow::Borrowed("mysimpletable"),
    );
    builder.gen_select_cte_with_counter(&table).unwrap();
    let res = builder.build().unwrap();

    insta::assert_debug_snapshot!(res);
}
