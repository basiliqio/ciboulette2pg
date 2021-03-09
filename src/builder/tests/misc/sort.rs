use super::*;

#[test]
fn multi() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store();
    let parsed_url = Url::parse(
        "http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2?sort=articles.title",
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

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let mut builder = Ciboulette2PostgresBuilder::default();
    let main_table = table_store.get("peoples").unwrap();
    let main_table_cte = main_table
        .to_cte(Cow::Owned(format!("cte_{}_main", main_table.name())))
        .unwrap();
    builder
        .gen_cte_for_sort(
            &ciboulette_store,
            &table_store,
            ciboulette_request.query(),
            ciboulette_store.get_type("peoples").unwrap(),
            &main_table,
            &main_table_cte,
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql(&res);
}

#[test]
fn single() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store();
    let parsed_url = Url::parse(
        "http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2?sort=favorite_color.color",
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

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &BODY);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteUpdateRequest::try_from(request).unwrap();
    let mut builder = Ciboulette2PostgresBuilder::default();
    let main_table = table_store.get("peoples").unwrap();
    let main_table_cte = main_table
        .to_cte(Cow::Owned(format!("cte_{}_main", main_table.name())))
        .unwrap();
    builder
        .gen_cte_for_sort(
            &ciboulette_store,
            &table_store,
            ciboulette_request.query(),
            ciboulette_store.get_type("peoples").unwrap(),
            &main_table,
            &main_table_cte,
        )
        .unwrap();

    let res = builder.build().unwrap();
    test_sql(&res);
}
