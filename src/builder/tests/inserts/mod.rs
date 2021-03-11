use super::*;

mod cte_final;
mod gen_json_builder;
mod gen_values;
mod insert;
mod insert_normal;
mod single_rel_cte;

fn gen_req_create_people<'a>(
    store: &'a CibouletteStore<'a>,
    parsed_url: &'a Url,
) -> CibouletteCreateRequest<'a> {
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
    let request = req_builder.build(&store).unwrap();
    CibouletteCreateRequest::try_from(request).unwrap()
}
