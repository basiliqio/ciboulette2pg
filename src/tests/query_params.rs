#[macro_export]
macro_rules! ciboulette_query_test_routine {
    ($name:ident, $transform_function:ident, $query_string:literal) => {
        #[ciboulette2postgres_test]
        async fn $name(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
            let data = init_values::init_values(&mut transaction).await;
            let raw_rows =
                $transform_function(&mut transaction, $query_string, stringify!($name), &data)
                    .await;
            let res = Ciboulette2PostgresRow::from_raw(&raw_rows)
                .expect("to deserialize the returned rows");
            check_rows!(res);
        }
    };

    ($name:ident, $transform_function:ident, $query_string:literal, $type_to_join:literal) => {
        #[ciboulette2postgres_test]
        async fn $name(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
            let data = init_values::init_values(&mut transaction).await;
            let obj_id = data.get($type_to_join).unwrap().first().unwrap();
            let raw_rows = $transform_function(
                &mut transaction,
                format!($query_string, obj_id).as_str(),
                stringify!($name),
                &data,
            )
            .await;
            let res = Ciboulette2PostgresRow::from_raw(&raw_rows)
                .expect("to deserialize the returned rows");
            check_rows!(res);
        }
    };
}

#[macro_export]
macro_rules! ciboulette_query_test_multi {
    ($transform_function:ident) => {
        ciboulette_query_test_routine!(sorting, $transform_function, "/peoples?&sort=first-name");
        ciboulette_query_test_routine!(
            sorting_full_path,
            $transform_function,
            "/peoples?sort=peoples.first-name"
        );
        ciboulette_query_test_routine!(sorting_desc, $transform_function, "/peoples?sort=-age");
        ciboulette_query_test_routine!(
            sorting_multiple_fields,
            $transform_function,
            "/peoples?sort=age,-first-name"
        );
        ciboulette_query_test_routine!(
            sorting_by_one_to_one_rel,
            $transform_function,
            "/peoples?sort=-favorite_color.color,first-name"
        );
    };
}

#[macro_export]
macro_rules! ciboulette_query_test_single {
    ($transform_function:ident) => {
        ciboulette_query_test_routine!(
			sparse,
			$transform_function,
			"/peoples/{}?fields[peoples]=first-name",
			"peoples"
		);
		ciboulette_query_test_routine!(
			include,
			$transform_function,
			"/peoples/{}?include=articles",
			"peoples"
		);
		ciboulette_query_test_routine!(
			sparse_others,
			$transform_function,
			"/peoples/{}?include=articles&fields[articles]=title",
			"peoples"
		);
		ciboulette_query_test_routine!(
			include_full_path,
			$transform_function,
			"/peoples/{}?include=peoples.articles",
			"peoples"
		);
		ciboulette_query_test_routine!(
			include_multiple_resources,
			$transform_function,
			"/peoples/{}?include=peoples.articles,people-article",
			"peoples"
		);
		ciboulette_query_test_routine!(include_multiple_resources_with_sparsing, $transform_function, "/peoples/{}?include=peoples.articles,people-article&fields[peoples]=last-name&fields[articles]=title&fields[people-article]=article_id,people_id", "peoples");
    };
}
