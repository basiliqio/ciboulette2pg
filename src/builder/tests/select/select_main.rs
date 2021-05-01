use super::*;

#[test]
fn all() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse("http://localhost/peoples").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteReadRequest::try_from(request).unwrap();
    let builder =
        Ciboulette2PgBuilder::gen_select(&ciboulette_store, &table_store, &ciboulette_request)
            .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn single_id() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url =
        Url::parse("http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteReadRequest::try_from(request).unwrap();
    let builder =
        Ciboulette2PgBuilder::gen_select(&ciboulette_store, &table_store, &ciboulette_request)
            .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}
