use super::*;

#[test]
fn simple() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse("http://localhost/peoples").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
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
    let builder =
        Ciboulette2PgBuilder::gen_insert(&ciboulette_store, &table_store, &ciboulette_request)
            .unwrap();
    let res = builder.build().unwrap();

    println!("{}", res.0);
    test_sql!(res);
}
#[test]
fn relationship_external() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url =
        Url::parse("http://localhost/peoples?include=favorite_color,articles,people-article")
            .unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"type": "peoples",
			"attributes":
			{
				"first-name": "Hello",
				"last-name": "World"
			},
			"relationships":
			{
				"articles":
				{
					"data":
					[
						{
							"id": "8759b021-0932-40cb-8386-e1a83da1c73e",
							"type": "articles"
						},
						{
							"id": "5043c5ef-debd-4b60-b905-7dab2a7338bc",
							"type": "articles"
						}
					]
				}
			}
		}
	}
	"#,
    );

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteCreateRequest::try_from(request).unwrap();
    let err =
        Ciboulette2PgBuilder::gen_insert(&ciboulette_store, &table_store, &ciboulette_request)
            .unwrap_err();
    assert_eq!(
        matches!(err, Ciboulette2PgError::ManyRelationshipDirectWrite),
        true
    );
}

#[test]
fn relationship_internal_single() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse("http://localhost/peoples").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"type": "peoples",
			"attributes":
			{
				"first-name": "Hello",
				"last-name": "World"
			},
			"relationships":
			{
				"favorite_color":
				{
					"data":
					{
						"id": "8759b021-0932-40cb-8386-e1a83da1c73e",
						"type": "favorite_color"
					}
				}
			}
		}
	}
	"#,
    );

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteCreateRequest::try_from(request).unwrap();
    let builder =
        Ciboulette2PgBuilder::gen_insert(&ciboulette_store, &table_store, &ciboulette_request)
            .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn relationship_internal_single_include() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse("http://localhost/peoples?include=favorite_color").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"type": "peoples",
			"attributes":
			{
				"first-name": "Hello",
				"last-name": "World"
			},
			"relationships":
			{
				"favorite_color":
				{
					"data":
					{
						"id": "8759b021-0932-40cb-8386-e1a83da1c73e",
						"type": "favorite_color"
					}
				}
			}
		}
	}
	"#,
    );

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteCreateRequest::try_from(request).unwrap();
    let builder =
        Ciboulette2PgBuilder::gen_insert(&ciboulette_store, &table_store, &ciboulette_request)
            .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn relationship_internal_single_include_all() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(
        "http://localhost/peoples?include=favorite_color,articles,people-article,comments",
    )
    .unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Create;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"type": "peoples",
			"attributes":
			{
				"first-name": "Hello",
				"last-name": "World"
			},
			"relationships":
			{
				"favorite_color":
				{
					"data":
					{
						"id": "8759b021-0932-40cb-8386-e1a83da1c73e",
						"type": "favorite_color"
					}
				}
			}
		}
	}
	"#,
    );

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteCreateRequest::try_from(request).unwrap();
    let builder =
        Ciboulette2PgBuilder::gen_insert(&ciboulette_store, &table_store, &ciboulette_request)
            .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}
