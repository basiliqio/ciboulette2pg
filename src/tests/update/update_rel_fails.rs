use super::*;

async fn test_update_fails<'a>(
    query_end: &str,
    body: &str,
) -> Ciboulette2SqlError {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(format!("http://localhost{}", query_end).as_str()).unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    let body: Option<&str> = Some(body);

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &body);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    Ciboulette2PostgresBuilder::gen_update(&ciboulette_store, &table_store, &ciboulette_request)
        .unwrap_err()
}

#[ciboulette2postgres_test]
async fn updating_many_to_many_rels(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let err = test_update_fails(
        format!("/peoples/{}/relationships/articles", people_id).as_str(),
        json!({ "data": serde_json::Value::Null })
            .to_string()
            .as_str(),
    )
    .await;
    assert_eq!(
        matches!(err, Ciboulette2SqlError::UpdatingManyRelationships),
        true
    );
}

#[ciboulette2postgres_test]
async fn updating_one_to_many_rels(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let err = test_update_fails(
        format!("/peoples/{}/relationships/comments", people_id).as_str(),
        json!({ "data": serde_json::Value::Null })
            .to_string()
            .as_str(),
    )
    .await;
    assert_eq!(
        matches!(err, Ciboulette2SqlError::UpdatingManyRelationships),
        true
    );
}
