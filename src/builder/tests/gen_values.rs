use super::*;

#[test]
fn simple() {
    let mut builder = Ciboulette2PostgresBuilder::new();
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
            "uuid",
        )
        .unwrap();
    let res = builder.build().unwrap();

    insta::assert_debug_snapshot!(res);
}
