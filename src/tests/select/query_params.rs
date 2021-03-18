use super::*;

macro_rules! ciboulette_query_test {
    ($name:ident, $transform_function:ident, $query_string:literal) => {
        #[ciboulette2postgres_test]
        async fn $name(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
            init_values::init_values(&mut transaction).await;
            let raw_rows = $transform_function(&mut transaction, $query_string).await;
            let res = Ciboulette2PostgresRow::from_raw(&raw_rows)
                .expect("to deserialize the returned rows");
            check_rows(&res);
        }
    };

    ($name:ident, $transform_function:ident, $query_string:literal, $type_to_join:literal) => {
        #[ciboulette2postgres_test]
        async fn $name(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
            let data = init_values::init_values(&mut transaction).await;
            let obj_id = data.get($type_to_join).unwrap().first().unwrap();
            let raw_rows =
                $transform_function(&mut transaction, format!($query_string, obj_id).as_str())
                    .await;
            let res = Ciboulette2PostgresRow::from_raw(&raw_rows)
                .expect("to deserialize the returned rows");
            check_rows(&res);
        }
    };
}
ciboulette_query_test!(
    sparse,
    test_select,
    "/peoples/{}?fields[peoples]=first-name",
    "peoples"
);
ciboulette_query_test!(
    include,
    test_select,
    "/peoples/{}?include=articles",
    "peoples"
);
ciboulette_query_test!(
    sparse_others,
    test_select,
    "/peoples/{}?include=articles&fields[articles]=title",
    "peoples"
);
ciboulette_query_test!(
    include_full_path,
    test_select,
    "/peoples/{}?include=peoples.articles",
    "peoples"
);
ciboulette_query_test!(
    include_multiple_resources,
    test_select,
    "/peoples/{}?include=peoples.articles,people-article",
    "peoples"
);
ciboulette_query_test!(include_multiple_resources_with_sparsing, test_select, "/peoples/{}?include=peoples.articles,people-article&fields[peoples]=last-name&fields[articles]=title&fields[people-article]=article_id,people_id", "peoples");
ciboulette_query_test!(sorting, test_select, "/peoples?&sort=first-name");
ciboulette_query_test!(
    sorting_full_path,
    test_select,
    "/peoples?sort=peoples.first-name"
);
ciboulette_query_test!(sorting_desc, test_select, "/peoples?sort=-age");
ciboulette_query_test!(
    sorting_multiple_fields,
    test_select,
    "/peoples?sort=age,-first-name"
);
ciboulette_query_test!(
    sorting_multiple_resources,
    test_select,
    "/peoples?sort=articles.title,first-name"
);
