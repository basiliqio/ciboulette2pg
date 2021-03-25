use super::*;

async fn test_insert<'a>(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    query_end: &str,
    body: &str,
) -> Vec<sqlx::postgres::PgRow> {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(format!("http://localhost{}", query_end).as_str()).unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    let body: Option<&str> = Some(body);

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &body);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteCreateRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_insert(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let (query, args) = builder.build().unwrap();

    let raw_rows: Vec<sqlx::postgres::PgRow> = sqlx::query_with(&query, args)
        .fetch_all(&mut *transaction)
        .await
        .unwrap();
    raw_rows
}

#[ciboulette2postgres_test]
async fn insert_main_all_fields(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let raw_rows = test_insert(
        &mut transaction,
        "/peoples",
        json!({
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
        .to_string()
        .as_str(),
    )
    .await;
    Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    snapshot_table(
        &mut transaction,
        "insert_main_all_fields",
        &["peoples", "people-article", "favorite_color"],
    )
    .await;
}

#[ciboulette2postgres_test]
async fn insert_main_required_only(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let raw_rows = test_insert(
        &mut transaction,
        "/peoples",
        json!({
            "data": json!({
                "type": "peoples",
                "attributes": json!({
                    "first-name": "Hello",
                    "last-name": "World"
                })
            })
        })
        .to_string()
        .as_str(),
    )
    .await;
    Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    snapshot_table(
        &mut transaction,
        "insert_main_required_fields",
        &["peoples", "people-article", "favorite_color"],
    )
    .await;
}

#[ciboulette2postgres_test]
async fn insert_main_with_favorite_color(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let raw_rows_color = test_insert(
        &mut transaction,
        "/favorite_color",
        json!({
            "data": json!({
                "type": "favorite_color",
                "attributes": json!({
                    "color": "red"
                })
            })
        })
        .to_string()
        .as_str(),
    )
    .await;
    let color_rows = Ciboulette2PostgresRow::from_raw(&raw_rows_color)
        .expect("to deserialize the returned rows");
    let raw_rows_main = test_insert(
        &mut transaction,
        "/peoples",
        json!({
            "data": json!({
                "type": "peoples",
                "attributes": json!({
                    "first-name": "Hello",
                    "last-name": "World"
                }),
                "relationships": json!({
                    "favorite_color": json!({
                        "data": json!({
                            "id": color_rows.first().unwrap().id(),
                            "type": "favorite_color"
                        })
                    })
                })
            })
        })
        .to_string()
        .as_str(),
    )
    .await;
    Ciboulette2PostgresRow::from_raw(&raw_rows_main).expect("to deserialize the returned rows");
    snapshot_table(
        &mut transaction,
        "insert_main_with_color",
        &["peoples", "people-article", "favorite_color"],
    )
    .await;
}
