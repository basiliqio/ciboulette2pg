use super::*;

fn test_insert_failing(body: String) -> Ciboulette2SqlError {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse("http://localhost/peoples").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    let body: Option<&str> = Some(body.as_str());

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &body);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteCreateRequest::try_from(request).unwrap();
    Ciboulette2PostgresBuilder::gen_insert(&ciboulette_store, &table_store, &ciboulette_request)
        .unwrap_err()
}

#[ciboulette2postgres_test]
async fn providing_id(_transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let err = test_insert_failing(
        json!({
            "data": json!({
                "type": "peoples",
                "id": "123b0f33-8531-43cb-9439-1b5bb7254503",
                "attributes": json!({
                    "first-name": "Hello",
                    "last-name": "World",
                    "twitter": "@myhandle",
                    "gender": "M",
                    "age": 19
                })
            })
        })
        .to_string(),
    );
    assert_eq!(
        matches!(err, Ciboulette2SqlError::ProvidedIdOnInserts),
        true
    );
}

#[ciboulette2postgres_test]
async fn missing_attributes(_transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let err = test_insert_failing(
        json!({
            "data": json!({
                "type": "peoples"
            })
        })
        .to_string(),
    );
    assert_eq!(matches!(err, Ciboulette2SqlError::MissingAttributes), true);
}
