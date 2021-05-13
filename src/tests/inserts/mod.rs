use super::*;

mod inserts_fails;
mod inserts_success;
mod query_params;

async fn test_insert<'store>(
    pool: &mut sqlx::PgPool,
    query_end: &str,
    _test_name: &str,
    _data: &BTreeMap<String, Vec<String>>,
) -> Vec<sqlx::postgres::PgRow> {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(format!("http://localhost{}", query_end).as_str()).unwrap();
    let body = json!({
        "data": json!({
            "type": "peoples",
            "attributes": json!({
                "first-name": "Hello",
                "last-name": "World",
                "twitter": "@myhandle",
                "gender": "M",
                "age": 19
            })
        })
    })
    .to_string();
    let body_opt = Some(body.as_str());
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &body_opt);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteCreateRequest::try_from(request).unwrap();
    let builder =
        Ciboulette2PgBuilder::gen_insert(&ciboulette_store, &table_store, &ciboulette_request)
            .unwrap();
    let (query, args) = builder.build().unwrap();

    let raw_rows: Vec<sqlx::postgres::PgRow> = sqlx::query_with(&query, args)
        .fetch_all(&mut pool.acquire().await.unwrap())
        .await
        .unwrap();
    snapshot_table(
        &mut *pool,
        "db_snapshot_insert_while_testing_query_params",
        &["peoples", "people-article", "favorite_color"],
    )
    .await;
    raw_rows
}
