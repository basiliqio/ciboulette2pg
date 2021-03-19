use super::*;

#[ciboulette2postgres_test]
async fn providing_id(_transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse("http://localhost/peoples").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    let body_json = json!({
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
    .to_string();
    let body: Option<&str> = Some(body_json.as_str());

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &body);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteCreateRequest::try_from(request).unwrap();
    let err = Ciboulette2PostgresBuilder::gen_insert(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap_err();
    assert_eq!(
        matches!(err, Ciboulette2SqlError::ProvidedIdOnInserts),
        true
    );
}
