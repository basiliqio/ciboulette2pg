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
    let builder =
        Ciboulette2PgBuilder::gen_update(&ciboulette_store, &table_store, &ciboulette_request)
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
async fn set_one_to_one(mut pool: sqlx::PgPool) {
    let data = basiliq_db_test_utils::init_values(&mut pool).await;
    baseline_for_people!(pool);
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let favorite_color = data.get("favorite_color").unwrap().last().unwrap();
    let raw_rows = test_update(
        &mut pool,
        format!("/peoples/{}/relationships/favorite_color", people_id).as_str(),
        json!({
            "data": json!({
                "type": "favorite_color",
                "id": favorite_color
            })
        })
        .to_string()
        .as_str(),
    )
    .await;
    let rows = Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    snapshot_table(&mut pool, "update_one_to_one", &["peoples"]).await;
    for row in rows.into_iter() {
        if row.type_() != &"favorite_color" {
            continue;
        }
        assert_eq!(row.id(), &favorite_color.to_string().as_str());
        return;
    }
    panic!("The relation hasn't been returned");
}

#[basiliq_test(run_migrations)]
async fn set_many_to_one(mut pool: sqlx::PgPool) {
    let data = basiliq_db_test_utils::init_values(&mut pool).await;
    baseline_for_people!(pool);
    let people_id = data.get("peoples").unwrap().last().unwrap();
    let comment_id = data.get("comments").unwrap().first().unwrap();
    let raw_rows = test_update(
        &mut pool,
        format!("/comments/{}/relationships/author", comment_id).as_str(),
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
    let rows = Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    snapshot_table(&mut pool, "update_many_to_one", &["comments"]).await;
    for row in rows.into_iter() {
        if row.type_() != &"author" {
            continue;
        }
        assert_eq!(row.id(), &people_id.to_string().as_str());
        return;
    }
    panic!("The relation hasn't been returned");
}

#[basiliq_test(run_migrations)]
async fn unset_one_to_one(mut pool: sqlx::PgPool) {
    let data = basiliq_db_test_utils::init_values(&mut pool).await;
    baseline_for_people!(pool);
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_update(
        &mut pool,
        format!("/peoples/{}/relationships/favorite_color", people_id).as_str(),
        json!({ "data": serde_json::Value::Null })
            .to_string()
            .as_str(),
    )
    .await;
    Ciboulette2PgRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    snapshot_table(&mut pool, "update_unset_one_to_one", &["peoples"]).await;
}
