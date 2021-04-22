use super::*;

#[test]
fn include_multi() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(
        "http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2?include=articles",
    )
    .unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteReadRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_select(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn include_nested() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(
        "http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2?include=articles.comments",
    )
    .unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteReadRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_select(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let res = builder.build().unwrap();
    // for el in res.1.iter().enumerate()
    // {
    // 	println!("[{}] {:#?}", el.0, el.1);
    // }
    test_sql!(res);
}

#[test]
fn include_single() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(
        "http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2?include=favorite_color",
    )
    .unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteReadRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_select(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let res = builder.build().unwrap();
    test_sql!(res);
}

#[test]
fn include_deep_nested() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse(
        "http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2?include=articles.comments.author.favorite_color",
    )
    .unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteReadRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_select(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let res = builder.build().unwrap();
    test_sql!(res);
}
