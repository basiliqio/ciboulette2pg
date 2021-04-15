use super::*;

mod query_params;
mod update_main_fails;
mod update_rel_fails;
mod update_rel_success;
mod update_success;

async fn test_update<'store>(
    pool: &mut sqlx::PgPool,
    query_end: &str,
    _test_name: &str,
    data: &BTreeMap<String, Vec<Uuid>>,
) -> Vec<sqlx::postgres::PgRow> {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(format!("http://localhost{}", query_end).as_str()).unwrap();
    let first_person = data.get("peoples").unwrap().first().unwrap();
    let body = json!({
        "data": json!({
            "type": "peoples",
            "id": first_person,
            "attributes": json!({
                "first-name": "Sicnarf",
                "last-name": "Yor el",
            })
        })
    })
    .to_string();
    let body_opt = Some(body.as_str());
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &body_opt);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_update(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let (query, args) = builder.build().unwrap();

    let raw_rows: Vec<sqlx::postgres::PgRow> = sqlx::query_with(&query, args)
        .fetch_all(&mut pool.acquire().await.unwrap())
        .await
        .unwrap();
    snapshot_table(
        &mut *pool,
        "db_snapshot_update_main_while_testing_query_params",
        &["peoples", "people-article", "favorite_color"],
    )
    .await;
    raw_rows
}

async fn test_update_many_to_one<'store>(
    pool: &mut sqlx::PgPool,
    query_end: &str,
    _test_name: &str,
    data: &BTreeMap<String, Vec<Uuid>>,
) -> Vec<sqlx::postgres::PgRow> {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(format!("http://localhost{}", query_end).as_str()).unwrap();
    let last_person = data.get("peoples").unwrap().last().unwrap();
    let body = json!({
        "data": json!({
            "type": "peoples",
            "id": last_person,
        })
    })
    .to_string();
    let body_opt = Some(body.as_str());
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &body_opt);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_update(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let (query, args) = builder.build().unwrap();

    let raw_rows: Vec<sqlx::postgres::PgRow> = sqlx::query_with(&query, args)
        .fetch_all(&mut pool.acquire().await.unwrap())
        .await
        .unwrap();
    snapshot_table(
        &mut *pool,
        "db_snapshot_update_rels_while_testing_query_params",
        &["comments"],
    )
    .await;
    raw_rows
}
