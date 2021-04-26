use super::*;

#[test]
fn empty() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url =
        Url::parse("http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "peoples"
		}
	}
	"#,
    );

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_update(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn simple() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url =
        Url::parse("http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "peoples",
			"attributes":
			{
				"first-name": "Bonjour",
				"last-name": "Le Monde"
			}
		}
	}
	"#,
    );

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_update(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn relationship_internal() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url =
        Url::parse("http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "peoples",
			"attributes":
			{
				"first-name": "Bonjour",
				"last-name": "Monde"
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

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_update(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn relationship_internal_force_null() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url =
        Url::parse("http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "peoples",
			"attributes":
			{
				"first-name": "Bonjour",
				"last-name": "Monde"
			},
			"relationships":
			{
				"favorite_color":
				{
					"data": null
				}
			}
		}
	}
	"#,
    );

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_update(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn relationship_external() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url =
        Url::parse("http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "peoples",
			"attributes":
			{
				"first-name": "Bonjour",
				"last-name": "Monde"
			},
			"relationships":
			{
				"articles":
				{
					"data":
					{
						"id": "8759b021-0932-40cb-8386-e1a83da1c73e",
						"type": "articles"
					}
				}
			}
		}
	}
	"#,
    );

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let err: Ciboulette2SqlError = Ciboulette2PostgresBuilder::gen_update(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap_err();

    assert_eq!(
        matches!(err, Ciboulette2SqlError::ManyRelationshipDirectWrite),
        true
    );
}

#[test]
fn not_all_required_fields() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url =
        Url::parse("http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "peoples",
			"attributes":
			{
				"first-name": "Bonjour"
			}
		}
	}
	"#,
    );

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_update(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();

    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn forced_null_fields() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url =
        Url::parse("http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "peoples",
			"attributes":
			{
				"first-name": "Bonjour",
				"gender": null
			}
		}
	}
	"#,
    );

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_update(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();

    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn include_single_rel() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(
        "http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2?include=favorite_color",
    )
    .unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "peoples",
			"attributes":
			{
				"first-name": "Bonjour",
				"last-name": "Le Monde"
			}
		}
	}
	"#,
    );

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_update(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();

    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn include_multi_rel() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(
        "http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2?include=articles",
    )
    .unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Update;
    const BODY: Option<&str> = Some(
        r#"
	{
		"data":
		{
			"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
			"type": "peoples",
			"attributes":
			{
				"first-name": "Bonjour",
				"last-name": "Le Monde"
			}
		}
	}
	"#,
    );

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_update(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();

    let res = builder.build().unwrap();

    test_sql!(res);
}
