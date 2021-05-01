use super::*;

#[test]
fn simple() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url =
        Url::parse("http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/favorite_color")
            .unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data": {
			"id": "5be39677-c0b4-4812-b85a-fbe903daab96",
			"type": "favorite_color"
		}
	}
	"#,
    );

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let builder =
        Ciboulette2PgBuilder::gen_update(&ciboulette_store, &table_store, &ciboulette_request)
            .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn forced_null() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url =
        Url::parse("http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/favorite_color")
            .unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data": null
	}
	"#,
    );

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let builder =
        Ciboulette2PgBuilder::gen_update(&ciboulette_store, &table_store, &ciboulette_request)
            .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn forbidden_multi_ids() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url =
        Url::parse("http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/favorite_color")
            .unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		[
			{
				"id": "5be39677-c0b4-4812-b85a-fbe903daab96",
				"type": "favorite_color"
			},
			{
				"id": "b4689d8f-8e81-4ccb-91e4-b28b47736fa7",
				"type": "favorite_color"
			}
		]
	}
	"#,
    );

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let err =
        Ciboulette2PgBuilder::gen_update(&ciboulette_store, &table_store, &ciboulette_request)
            .unwrap_err();

    assert_eq!(
        matches!(err, Ciboulette2PgError::MultiIdsForSingleRelationships),
        true
    );
}

#[test]
fn forbidden_one_to_many() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(
        "http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/articles",
    )
    .unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
			{
				"id": "b4689d8f-8e81-4ccb-91e4-b28b47736fa7",
				"type": "articles"
			}
	}
	"#,
    );

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let err =
        Ciboulette2PgBuilder::gen_update(&ciboulette_store, &table_store, &ciboulette_request)
            .unwrap_err();

    assert_eq!(
        matches!(err, Ciboulette2PgError::ManyRelationshipDirectWrite),
        true
    );
}
