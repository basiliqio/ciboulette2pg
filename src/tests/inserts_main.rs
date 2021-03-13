use super::*;
use sqlx::Row;

#[ciboulette2postgres_test]
async fn insert_main_all_fields(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse("http://localhost/peoples").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "peoples",
			"attributes":
			{
				"first-name": "Hello",
				"last-name": "World"
			}
		}
	}
	"#,
    );

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteCreateRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_insert(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let (query, args) = builder.build().unwrap();

    let res = sqlx::query_with(&query, args)
        .fetch_all(&mut transaction)
        .await
        .unwrap();
    let row: &sqlx::postgres::PgRow = res.get(0).unwrap();
    let id: &str = row.try_get(0).unwrap();
    let type_: &str = row.try_get(1).unwrap();
    let data: serde_json::Value = row.try_get(2).unwrap();
    println!("{} | {} | {}", id, type_, data);
    // insta::assert_debug_snapshot!(res);
}

#[tokio::test]
async fn normal() {
    assert_eq!(true, true);
}
