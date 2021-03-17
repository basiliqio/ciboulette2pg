use super::*;

#[ciboulette2postgres_test]
async fn sparse(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples/{}?fields[peoples]=first-name", people_id).as_str(),
    )
    .await;
    let res =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows(&res);
}

#[ciboulette2postgres_test]
async fn include(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples/{}?include=articles", people_id).as_str(),
    )
    .await;
    let res =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows(&res);
}

#[ciboulette2postgres_test]
async fn include_full_path(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples/{}?include=peoples.articles", people_id).as_str(),
    )
    .await;
    let res =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows(&res);
}

#[ciboulette2postgres_test]
async fn include_multiple_resources(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(
        &mut transaction,
        format!(
            "/peoples/{}?include=peoples.articles,people-article",
            people_id
        )
        .as_str(),
    )
    .await;
    let res =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows(&res);
}

#[ciboulette2postgres_test]
async fn include_multiple_resources_with_sparsing(
    mut transaction: sqlx::Transaction<'_, sqlx::Postgres>
) {
    let data = init_values::init_values(&mut transaction).await;
    let people_id = data.get("peoples").unwrap().first().unwrap();
    let raw_rows = test_select(&mut transaction, format!("/peoples/{}?include=peoples.articles,people-article&fields[peoples]=last-name&fields[articles]=title&fields[people-article]=article_id,people_id", people_id).as_str()).await;
    let res =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows(&res);
}

#[ciboulette2postgres_test]
async fn sorting(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    init_values::init_values(&mut transaction).await;
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples?&sort=first-name").as_str(),
    )
    .await;
    let res =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows(&res);
}

#[ciboulette2postgres_test]
async fn sorting_full_path(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    init_values::init_values(&mut transaction).await;
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples?sort=peoples.first-name").as_str(),
    )
    .await;
    let res =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows(&res);
}

#[ciboulette2postgres_test]
async fn sorting_desc(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    init_values::init_values(&mut transaction).await;
    let raw_rows = test_select(&mut transaction, format!("/peoples?sort=-age").as_str()).await;
    let res =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows(&res);
}

#[ciboulette2postgres_test]
async fn sorting_multiple_fields(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    init_values::init_values(&mut transaction).await;
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples?sort=age,-first-name").as_str(),
    )
    .await;
    let res =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    check_rows(&res);
}

#[ciboulette2postgres_test]
async fn sorting_multiple_resources(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    init_values::init_values(&mut transaction).await;
    let raw_rows = test_select(
        &mut transaction,
        format!("/peoples?include=articles&sort=articles.title,first-name").as_str(),
    )
    .await;
    let res =
        Ciboulette2PostgresRow::from_raw(&raw_rows).expect("to deserialize the returned rows");
    let res = res
        .into_iter()
        .filter(|x| x.type_() == &"peoples")
        .collect(); // FIXME don't
    check_rows(&res);
}
