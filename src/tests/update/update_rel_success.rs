use super::*;

async fn test_update<'a>(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
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
        .fetch_all(&mut *transaction)
        .await
        .unwrap();
    raw_rows
}

macro_rules! baseline_for_people {
    ($transaction:ident) => {
        snapshot_table(&mut $transaction, "normal_people_table", &["peoples"]).await;
    };
}

#[ciboulette2postgres_test]
async fn set_one_to_one(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    baseline_for_people!(transaction);
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let favorite_color = data.get("favorite_color").unwrap().last().unwrap();
    let raw_rows = test_update(
        &mut transaction,
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
    let rows =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    snapshot_table(&mut transaction, "update_one_to_one", &["peoples"]).await;
    for row in rows.into_iter() {
        if row.type_() != &"favorite_color" {
            continue;
        }
        assert_eq!(row.id(), &favorite_color.to_string().as_str());
        return;
    }
    panic!("The relation hasn't been returned");
}

#[ciboulette2postgres_test]
async fn set_many_to_one(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    baseline_for_people!(transaction);
    let people_id = data.get("peoples").unwrap().last().unwrap();
    let comment_id = data.get("comments").unwrap().first().unwrap();
    let raw_rows = test_update(
        &mut transaction,
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
    let rows =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    snapshot_table(&mut transaction, "update_many_to_one", &["comments"]).await;
    for row in rows.into_iter() {
        if row.type_() != &"peoples" {
            continue;
        }
        assert_eq!(row.id(), &people_id.to_string().as_str());
        return;
    }
    panic!("The relation hasn't been returned");
}

#[ciboulette2postgres_test]
async fn unset_one_to_one(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    baseline_for_people!(transaction);
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_update(
        &mut transaction,
        format!("/peoples/{}/relationships/favorite_color", people_id).as_str(),
        json!({ "data": serde_json::Value::Null })
            .to_string()
            .as_str(),
    )
    .await;
    Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    snapshot_table(&mut transaction, "update_unset_one_to_one", &["peoples"]).await;
}
