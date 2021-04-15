use super::*;

async fn test_delete<'store>(
    pool: &mut sqlx::PgPool,
    query_end: &str,
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
    raw_rows
}

macro_rules! baseline {
    ($pool:ident) => {
        snapshot_table(
            &mut $pool,
            "normal_people_table_and_people_article",
            &["peoples", "people-article"],
        )
        .await;
    };
}

#[basiliq_test(run_migrations)]
async fn main(mut pool: sqlx::PgPool) {
    let data = init_values::init_values(&mut pool).await;
    baseline!(pool);
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_delete(&mut pool, format!("/peoples/{}", people_id).as_str()).await;
    Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    snapshot_table(&mut pool, "delete_main", &["peoples", "people-article"]).await;
}

#[basiliq_test(run_migrations)]
async fn one_to_one(mut pool: sqlx::PgPool) {
    let data = init_values::init_values(&mut pool).await;
    baseline!(pool);
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_delete(
        &mut pool,
        format!("/peoples/{}/relationships/favorite_color", people_id).as_str(),
    )
    .await;
    Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    snapshot_table(
        &mut pool,
        "delete_one_to_one",
        &["peoples", "people-article"],
    )
    .await;
}
