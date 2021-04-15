use super::*;

mod delete_fails;
mod delete_success;
mod query_params;

async fn test_delete<'store>(
    pool: &mut sqlx::PgPool,
    query_end: &str,
    _test_name: &str,
    _data: &BTreeMap<String, Vec<Uuid>>,
) -> Vec<sqlx::postgres::PgRow> {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(format!("http://localhost{}", query_end).as_str()).unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Delete;

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteDeleteRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_delete(
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
        "db_snapshot_delete_while_testing_query_params",
        &["peoples", "people-article", "favorite_color"],
    )
    .await;
    raw_rows
}
