use super::*;

#[test]
fn multi() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse("http://localhost/peoples?sort=articles.title").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let req_builder = CibouletteInboundRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteReadRequest::try_from(request).unwrap();
    let error = Ciboulette2PostgresBuilder::gen_select(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap_err();

    assert_eq!(
        matches!(error, Ciboulette2SqlError::SortingByMultiRel(x, y) if x == "peoples" && y == "articles"),
        true
    );
}

#[test]
fn single() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse("http://localhost/peoples?sort=favorite_color.color").unwrap();
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
