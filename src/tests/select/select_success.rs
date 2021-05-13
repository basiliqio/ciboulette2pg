use super::*;
#[basiliq_test(run_migrations)]
async fn select_all_fields(mut pool: sqlx::PgPool) {
    let data = basiliq_db_test_utils::init_values(&mut pool).await;
    let raw_rows = test_select(&mut pool, "/peoples", "", &data).await;
    let res = Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows!(res);
}

#[basiliq_test(run_migrations)]
async fn select_a_single_record(mut pool: sqlx::PgPool) {
    let data = basiliq_db_test_utils::init_values(&mut pool).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut pool,
        format!("/peoples/{}", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows!(res);
}

#[basiliq_test(run_migrations)]
async fn select_related_record_single_rels(mut pool: sqlx::PgPool) {
    let data = basiliq_db_test_utils::init_values(&mut pool).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut pool,
        format!("/peoples/{}/favorite_color", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows!(res);
}

#[basiliq_test(run_migrations)]
async fn select_related_record_multi_rels(mut pool: sqlx::PgPool) {
    let data = basiliq_db_test_utils::init_values(&mut pool).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut pool,
        format!("/peoples/{}/articles", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows!(res);
}

#[basiliq_test(run_migrations)]
async fn select_related_record_multi_rels_reverse(mut pool: sqlx::PgPool) {
    let data = basiliq_db_test_utils::init_values(&mut pool).await;
    let people_id = data.get("articles").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut pool,
        format!("/articles/{}/author", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows!(res);
}

#[basiliq_test(run_migrations)]
async fn select_one_to_many_rels(mut pool: sqlx::PgPool) {
    let data = basiliq_db_test_utils::init_values(&mut pool).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut pool,
        format!("/peoples/{}/comments", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows!(res);
}

#[basiliq_test(run_migrations)]
async fn select_many_to_one_rels(mut pool: sqlx::PgPool) {
    let data = basiliq_db_test_utils::init_values(&mut pool).await;
    let comment_id = data.get("comments").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut pool,
        format!("/comments/{}/author", comment_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows!(res);
}

#[basiliq_test(run_migrations)]
async fn select_one_to_one_relationships(mut pool: sqlx::PgPool) {
    let data = basiliq_db_test_utils::init_values(&mut pool).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut pool,
        format!("/peoples/{}/relationships/favorite_color", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows!(res);
}

#[basiliq_test(run_migrations)]
async fn select_one_to_many_relationships(mut pool: sqlx::PgPool) {
    let data = basiliq_db_test_utils::init_values(&mut pool).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut pool,
        format!("/peoples/{}/relationships/comments", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows!(res);
}

#[basiliq_test(run_migrations)]
async fn select_many_to_one_relationships(mut pool: sqlx::PgPool) {
    let data = basiliq_db_test_utils::init_values(&mut pool).await;
    let people_id = data.get("comments").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut pool,
        format!("/comments/{}/relationships/author", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows!(res);
}

#[basiliq_test(run_migrations)]
async fn select_many_to_many_relationships(mut pool: sqlx::PgPool) {
    let data = basiliq_db_test_utils::init_values(&mut pool).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut pool,
        format!("/peoples/{}/relationships/articles", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows!(res);
}
