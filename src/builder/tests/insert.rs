use super::*;

#[test]
fn simple() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store();
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
    let res = builder.build().unwrap();

    insta::assert_debug_snapshot!(res);
}

#[test]
fn relationship_external_single() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store();
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

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteCreateRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_insert(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let res = builder.build().unwrap();

    insta::assert_debug_snapshot!(res);
}