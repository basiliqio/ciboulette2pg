use super::*;

#[ciboulette2postgres_test]
async fn select_all_fields(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let raw_rows = test_select(&mut transaction, "/peoples", "", &data).await;
    let res = Ciboulette2PostgresRowBuilder::from_raw(&raw_rows)
        .expect("to deserialize the returned rows");
    check_rows!(res);
}

#[ciboulette2postgres_test]
async fn select_a_single_record(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples/{}", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PostgresRowBuilder::from_raw(&raw_rows)
        .expect("to deserialize the returned rows");
    check_rows!(res);
}

#[ciboulette2postgres_test]
async fn select_related_record_single_rels(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples/{}/favorite_color", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PostgresRowBuilder::from_raw(&raw_rows)
        .expect("to deserialize the returned rows");
    check_rows!(res);
}

#[ciboulette2postgres_test]
async fn select_related_record_multi_rels(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples/{}/articles", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PostgresRowBuilder::from_raw(&raw_rows)
        .expect("to deserialize the returned rows");
    check_rows!(res);
}

#[ciboulette2postgres_test]
async fn select_related_record_multi_rels_reverse(
    mut transaction: sqlx::Transaction<'_, sqlx::Postgres>
) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("articles").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!("/articles/{}/author", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PostgresRowBuilder::from_raw(&raw_rows)
        .expect("to deserialize the returned rows");
    check_rows!(res);
}

#[ciboulette2postgres_test]
async fn select_one_to_many_rels(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples/{}/comments", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PostgresRowBuilder::from_raw(&raw_rows)
        .expect("to deserialize the returned rows");
    check_rows!(res);
}

#[ciboulette2postgres_test]
async fn select_many_to_one_rels(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let comment_id = data.get("comments").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!("/comments/{}/author", comment_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PostgresRowBuilder::from_raw(&raw_rows)
        .expect("to deserialize the returned rows");
    check_rows!(res);
}

#[ciboulette2postgres_test]
async fn select_one_to_one_relationships(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples/{}/relationships/favorite_color", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PostgresRowBuilder::from_raw(&raw_rows)
        .expect("to deserialize the returned rows");
    check_rows!(res);
}

#[ciboulette2postgres_test]
async fn select_one_to_many_relationships(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples/{}/relationships/comments", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PostgresRowBuilder::from_raw(&raw_rows)
        .expect("to deserialize the returned rows");
    check_rows!(res);
}

#[ciboulette2postgres_test]
async fn select_many_to_one_relationships(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("comments").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!("/comments/{}/relationships/author", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PostgresRowBuilder::from_raw(&raw_rows)
        .expect("to deserialize the returned rows");
    check_rows!(res);
}

#[ciboulette2postgres_test]
async fn select_many_to_many_relationships(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples/{}/relationships/articles", people_id).as_str(),
        "",
        &data,
    )
    .await;
    let res = Ciboulette2PostgresRowBuilder::from_raw(&raw_rows)
        .expect("to deserialize the returned rows");
    check_rows!(res);
}
