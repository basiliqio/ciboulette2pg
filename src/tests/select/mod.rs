use super::*;

mod query_params;
mod select_success;

pub(crate) async fn test_select<'store>(
    pool: &mut sqlx::PgPool,
    query_end: &str,
    name: &str,
    _data: &BTreeMap<String, Vec<String>>,
) -> Vec<sqlx::postgres::PgRow> {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(format!("http://localhost{}", query_end).as_str()).unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteReadRequest::try_from(request).unwrap();
    let builder =
        Ciboulette2PgBuilder::gen_select(&ciboulette_store, &table_store, &ciboulette_request)
            .unwrap();
    let (query, args) = builder.build().unwrap();
    insta::assert_snapshot!(
        format!("{}_select_query", name),
        sqlformat::format(
            query.as_str(),
            &sqlformat::QueryParams::None,
            sqlformat::FormatOptions::default()
        )
    );
    let raw_rows: Vec<sqlx::postgres::PgRow> = sqlx::query_with(&query, args)
        .fetch_all(&mut pool.acquire().await.unwrap())
        .await
        .unwrap();
    raw_rows
}
