use super::*;

#[test]
fn many_to_many() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse("http://localhost/peoples?sort=articles.title").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteReadRequest::try_from(request).unwrap();
    let err = Ciboulette2PostgresBuilder::gen_select(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap_err();
    assert_eq!(
        matches!(err, Ciboulette2SqlError::SortingByMultiRel(_, _)),
        true
    );
}

#[test]
fn one_to_many() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse("http://localhost/articles?sort=comments.body").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteReadRequest::try_from(request).unwrap();
    let err = Ciboulette2PostgresBuilder::gen_select(
        &ciboulette_store,
        &table_store,
        &ciboulette_request,
    )
    .unwrap_err();
    assert_eq!(
        matches!(err, Ciboulette2SqlError::SortingByMultiRel(_, _)),
        true
    );
}

#[test]
fn single() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store(&ciboulette_store);
    let parsed_url = Url::parse("http://localhost/peoples?sort=favorite_color.color").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Read;

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
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
