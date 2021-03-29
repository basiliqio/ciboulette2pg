use super::select::test_select;
use super::*;

#[ciboulette2postgres_test]
async fn convert_multiple_field_without_related(
    mut transaction: sqlx::Transaction<'_, sqlx::Postgres>
) {
    let store = gen_bag();
    let data = init_values::init_values(&mut transaction).await;
    let raw_rows = test_select(&mut transaction, "/peoples", "", &data).await;
    let res =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    let hint_size = res.len();
    let res_built =
        Ciboulette2PostgresRow::build_response_elements(res, &store, Some(hint_size)).unwrap();
    check_response_elements!(res_built);
}

#[ciboulette2postgres_test]
async fn convert_multiple_field_with_related(
    mut transaction: sqlx::Transaction<'_, sqlx::Postgres>
) {
    let store = gen_bag();
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples/{}/articles?include=peoples", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    println!("{:#?}", res);
    let hint_size = res.len();
    let res_built =
        Ciboulette2PostgresRow::build_response_elements(res, &store, Some(hint_size)).unwrap();
    check_response_elements!(res_built);
}
