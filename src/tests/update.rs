use super::*;
// async fn test_insert<'a>(
//     transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
//     query_end: &str,
//     body: &str,
// ) -> Vec<sqlx::postgres::PgRow> {
//     let ciboulette_store = gen_bag();
//     let table_store = gen_table_store(&ciboulette_store);
//     let parsed_url = Url::parse(format!("http://localhost{}", query_end).as_str()).unwrap();
//     const INTENTION: CibouletteIntention = CibouletteIntention::Create;
//     let body: Option<&str> = Some(body);

//     let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &body);
//     let request = req_builder.build(&ciboulette_store).unwrap();
//     let ciboulette_request = CibouletteCreateRequest::try_from(request).unwrap();
//     let builder = Ciboulette2PostgresBuilder::gen_insert(
//         &ciboulette_store,
//         &table_store,
//         &ciboulette_request,
//     )
//     .unwrap();
//     let (query, args) = builder.build().unwrap();

//     let raw_rows: Vec<sqlx::postgres::PgRow> = sqlx::query_with(&query, args)
//         .fetch_all(&mut *transaction)
//         .await
//         .unwrap();
//     raw_rows
// }

#[ciboulette2postgres_test]
async fn insert_main_all_fields(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    init_values::init_values(&mut transaction).await;
}
