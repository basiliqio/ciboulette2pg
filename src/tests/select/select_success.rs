use super::*;

#[ciboulette2postgres_test]
async fn select_all_fields(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    init_values::init_values(&mut transaction).await;
    let raw_rows = test_select(&mut transaction, "/peoples", "").await;
    let res =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows!(res);
}

#[ciboulette2postgres_test]
async fn select_a_single_record(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples/{}", people_id).as_str(),
        "",
    )
    .await;
    let res =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows!(res);
}
