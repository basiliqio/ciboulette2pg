use super::*;

#[test]
fn simple_single_rel() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store();
    let parsed_url =
		Url::parse("http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/favorite_color").unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Delete;

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteDeleteRequest::try_from(request).unwrap();
    let mut builder = Ciboulette2PostgresBuilder::default();
    builder
        .gen_delete(&ciboulette_store, &table_store, &ciboulette_request)
        .unwrap();
    let res = builder.build().unwrap();

    test_sql!(res);
}

#[test]
fn simple_single_rel_non_optional() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store();
    let parsed_url = Url::parse(
        "http://localhost/comments/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/author",
    )
    .unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Delete;

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteDeleteRequest::try_from(request).unwrap();
    let mut builder = Ciboulette2PostgresBuilder::default();
    let err = builder
        .gen_delete(&ciboulette_store, &table_store, &ciboulette_request)
        .unwrap_err();
    assert_eq!(
        matches!(err, Ciboulette2SqlError::RequiredRelationship(x, y) if x == "comments" && y == "author_id"),
        true
    );
}

#[test]
fn multi_relationships() {
    let ciboulette_store = gen_bag();
    let table_store = gen_table_store();
    let parsed_url = Url::parse(
        "http://localhost/peoples/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/articles",
    )
    .unwrap();
    const INTENTION: CibouletteIntention = CibouletteIntention::Delete;

    let req_builder = CibouletteRequestBuilder::new(INTENTION, &parsed_url, &None);
    let request = req_builder.build(&ciboulette_store).unwrap();
    let ciboulette_request = CibouletteDeleteRequest::try_from(request).unwrap();
    let mut builder = Ciboulette2PostgresBuilder::default();
    let err = builder
        .gen_delete(&ciboulette_store, &table_store, &ciboulette_request)
        .unwrap_err();
    assert_eq!(
        matches!(err, Ciboulette2SqlError::BulkRelationshipDelete),
        true
    );
}
