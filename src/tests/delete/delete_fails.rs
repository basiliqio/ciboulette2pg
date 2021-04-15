use super::*;

async fn test_delete_fails<'store>(query_end: &str) -> Ciboulette2SqlError {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(format!("http://localhost{}", query_end).as_str()).unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Delete;

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteDeleteRequest::try_from(request).unwrap();
    Ciboulette2PostgresBuilder::gen_delete(&ciboulette_store, &table_store, &ciboulette_request)
        .unwrap_err()
}
#[basiliq_test(run_migrations)]
async fn one_to_many(mut pool: sqlx::PgPool) {
    let data = init_values::init_values(&mut pool).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let err =
        test_delete_fails(format!("/peoples/{}/relationships/articles", people_id).as_str()).await;
    assert_eq!(
        matches!(err, Ciboulette2SqlError::BulkRelationshipDelete),
        true
    );
}
