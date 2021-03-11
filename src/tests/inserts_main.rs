use super::*;

#[ciboulette2postgres_test]
async fn insert_main_all_fields(pool: PgPool) {
    println!("{:#?}", pool);
    assert_eq!(true, true);
}

#[tokio::test]
async fn normal() {
    assert_eq!(true, true);
}
