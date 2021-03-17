use super::*;

mod query_params;
mod select_success;

pub fn check_rows(rows: &Vec<Ciboulette2PostgresRow>) {
    let value = serde_json::to_value(rows).unwrap();

    insta::assert_json_snapshot!(value,
    {
        "[].id" => insta::dynamic_redaction(check_uuid),
        "[].data.article_id" => insta::dynamic_redaction(check_uuid),
        "[].data.people_id" => insta::dynamic_redaction(check_uuid)
    });
}

async fn test_select<'a>(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    query_end: &str,
) -> Vec<sqlx::postgres::PgRow> {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(format!("http://localhost{}", query_end).as_str()).unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteReadRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_select_normal(
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