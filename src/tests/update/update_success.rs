use super::*;

async fn test_update<'store>(
    pool: &mut sqlx::PgPool,
    query_end: &str,
    body: &str,
) -> Vec<sqlx::postgres::PgRow> {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(format!("http://localhost{}", query_end).as_str()).unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    let body: Option<&str> = Some(body);

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &body);
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
    raw_rows
}

macro_rules! baseline_for_people {
    ($pool:ident) => {
        snapshot_table(&mut $pool, "normal_people_table", &["peoples"]).await;
    };
}

#[basiliq_test(run_migrations)]
async fn empty(mut pool: sqlx::PgPool) {
    let data = init_values::init_values(&mut pool).await;
    baseline_for_people!(pool);
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_update(
        &mut pool,
        format!("/peoples/{}", people_id).as_str(),
        json!({
            "data": json!({
                "type": "peoples",
                "id": people_id
            })
        })
        .to_string()
        .as_str(),
    )
    .await;
    Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    snapshot_table(&mut pool, "update_empty", &["peoples"]).await;
}

#[basiliq_test(run_migrations)]
async fn main_fields(mut pool: sqlx::PgPool) {
    let data = init_values::init_values(&mut pool).await;
    baseline_for_people!(pool);
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_update(
        &mut pool,
        format!("/peoples/{}", people_id).as_str(),
        json!({
            "data": json!({
                "type": "peoples",
                "id": people_id,
                "attributes": json!({
                    "first-name": "New first",
                    "last-name": "New last name",
                })
            })
        })
        .to_string()
        .as_str(),
    )
    .await;
    Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    snapshot_table(&mut pool, "update_main_fields", &["peoples"]).await;
}

#[basiliq_test(run_migrations)]
async fn single_rel(mut pool: sqlx::PgPool) {
    let data = init_values::init_values(&mut pool).await;
    baseline_for_people!(pool);
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_update(
        &mut pool,
        format!("/peoples/{}", people_id).as_str(),
        json!({
            "data": json!({
                "type": "peoples",
                "id": people_id,
                "attributes": json!({
                    "first-name": "New first",
                    "last-name": "New last name",
                }),
                "relationships": json!({
                    "favorite_color": json!({
                        "data": json!({
                            "id": data.get("favorite_color").unwrap().last().unwrap(),
                            "type": "favorite_color"
                        })
                    }),
                }),
            })
        })
        .to_string()
        .as_str(),
    )
    .await;
    Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    snapshot_table(&mut pool, "update_single_rel_with_fields", &["peoples"]).await;
}

#[basiliq_test(run_migrations)]
async fn single_rel_unset(mut pool: sqlx::PgPool) {
    let data = init_values::init_values(&mut pool).await;
    baseline_for_people!(pool);
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_update(
        &mut pool,
        format!("/peoples/{}", people_id).as_str(),
        json!({
            "data": json!({
                "type": "peoples",
                "id": people_id,
                "attributes": json!({
                    "first-name": "New first",
                    "last-name": "New last name",
                }),
                "relationships": json!({
                    "favorite_color": json!({
                        "data": serde_json::Value::Null
                    }),
                }),
            })
        })
        .to_string()
        .as_str(),
    )
    .await;
    Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    snapshot_table(
        &mut pool,
        "update_single_rel_unset_with_fields",
        &["peoples"],
    )
    .await;
}

#[basiliq_test(run_migrations)]
async fn unsetting_a_field(mut pool: sqlx::PgPool) {
    let data = init_values::init_values(&mut pool).await;
    baseline_for_people!(pool);
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_update(
        &mut pool,
        format!("/peoples/{}", people_id).as_str(),
        json!({
            "data": json!({
                "type": "peoples",
                "id": people_id,
                "attributes": json!({
                    "gender": serde_json::Value::Null
                })
            })
        })
        .to_string()
        .as_str(),
    )
    .await;
    Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    snapshot_table(&mut pool, "unsetting_main_field", &["peoples"]).await;
}
