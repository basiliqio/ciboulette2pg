use super::*;

#[test]
fn simple() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store();
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

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_update_main(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let res = builder.build().unwrap();

    insta::assert_debug_snapshot!(res);
}

#[test]
fn relationship_internal() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store();
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

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_update_main(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let res = builder.build().unwrap();

    insta::assert_debug_snapshot!(res);
}
