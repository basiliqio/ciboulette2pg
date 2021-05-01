use super::select::test_select;
use super::*;

#[basiliq_test(run_migrations)]
async fn convert_multiple_field_without_related(mut pool: sqlx::PgPool) {
    let store = gen_bag();
    let data = init_values::init_values(&mut pool).await;
    let raw_rows = test_select(&mut pool, "/peoples", "", &data).await;
    let res = Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    let hint_size = res.len();
    let res_built = Ciboulette2PgRow::build_response_elements(
        res,
        &store,
        store.get_type("peoples").unwrap(),
        Some(hint_size),
    )
    .unwrap();
    check_response_elements!(res_built);
}

#[basiliq_test(run_migrations)]
async fn convert_multiple_field_with_related(mut pool: sqlx::PgPool) {
    let store = gen_bag();
    let data = init_values::init_values(&mut pool).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut pool,
        format!("/peoples/{}/articles?include=author", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    let hint_size = res.len();
    let res_built = Ciboulette2PgRow::build_response_elements(
        res,
        &store,
        store.get_type("articles").unwrap(),
        Some(hint_size),
    )
    .unwrap();
    check_response_elements!(res_built);
}
