use super::*;
use uuid::Uuid;

#[test]
fn simple_multi_rels() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let url_str = format!("http://localhost/peoples/{}/articles", Uuid::new_v4());
    let parsed_url = Url::parse(url_str.as_str()).unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteReadRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_select_normal(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn simple_single_rel() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let url_str = format!("http://localhost/peoples/{}/favorite_color", Uuid::new_v4());
    let parsed_url = Url::parse(url_str.as_str()).unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteReadRequest::try_from(request).unwrap();
    let builder = Ciboulette2PostgresBuilder::gen_select_normal(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}
