#[macro_export]
macro_rules! ciboulette_query_test_routine {
    ($name:ident, $transform_function:ident, $query_string:literal) => {
        #[basiliq_test(run_migrations)]
        async fn $name(mut pool: sqlx::PgPool) {
            let data = init_values::init_values(&mut pool).await;
            let raw_rows =
                $transform_function(&mut pool, $query_string, stringify!($name), &data).await;
            let res = Ciboulette2PostgresRow::from_raw(&raw_rows)
                .expect("to deserialize the returned rows");
            check_rows!(res);
        }
    };

    ($name:ident, $transform_function:ident, $query_string:literal, $type_to_join:literal) => {
        #[basiliq_test(run_migrations)]
        async fn $name(mut pool: sqlx::PgPool) {
            let data = init_values::init_values(&mut pool).await;
            let obj_id = data.get($type_to_join).unwrap().first().unwrap();
            let raw_rows = $transform_function(
                &mut pool,
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
macro_rules! ciboulette_query_test_related {
    ($transform_function:ident) => {
        ciboulette_query_test_routine!(
            related_sorting_by_child_full_path,
            $transform_function,
            "/peoples/{}/articles?sort=articles.body",
            "peoples"
        );
        ciboulette_query_test_routine!(
            related_sorting_by_child,
            $transform_function,
            "/peoples/{}/articles?sort=title",
            "peoples"
        );
        ciboulette_query_test_routine!(
            related_sparse_child,
            $transform_function,
            "/peoples/{}/articles?fields[articles]=title",
            "peoples"
        );
        ciboulette_query_test_routine!(
            related_sort_and_sparse,
            $transform_function,
            "/peoples/{}/articles?fields[articles]=title&sort=articles.body",
            "peoples"
        );
        ciboulette_query_test_routine!(
            related_include_root,
            $transform_function,
            "/peoples/{}/articles?include=peoples",
            "peoples"
        );
        ciboulette_query_test_routine!(
            related_include_and_sparse,
            $transform_function,
            "/peoples/{}/articles?include=peoples&fields[peoples]=first-name",
            "peoples"
        );
    };
}

#[macro_export]
macro_rules! ciboulette_query_test_relationship_many_to_many {
    ($transform_function:ident) => {
        ciboulette_query_test_routine!(
            relationships_many_to_many_sorting_by_child_full_path,
            $transform_function,
            "/peoples/{}/relationships/articles?sort=articles.body",
            "peoples"
        );
        ciboulette_query_test_routine!(
            relationships_many_to_many_sorting_by_child,
            $transform_function,
            "/peoples/{}/relationships/articles?sort=title",
            "peoples"
        );
        ciboulette_query_test_routine!(
            relationships_many_to_many_sparse_child,
            $transform_function,
            "/peoples/{}/relationships/articles?fields[articles]=title",
            "peoples"
        );
        ciboulette_query_test_routine!(
            relationships_many_to_many_sort_and_sparse,
            $transform_function,
            "/peoples/{}/relationships/articles?fields[articles]=title&sort=articles.body",
            "peoples"
        );
        ciboulette_query_test_routine!(
            relationships_many_to_many_include_root,
            $transform_function,
            "/peoples/{}/relationships/articles?include=peoples",
            "peoples"
        );
        ciboulette_query_test_routine!(
            relationships_many_to_many_include_and_sparse,
            $transform_function,
            "/peoples/{}/relationships/articles?include=peoples&fields[peoples]=first-name",
            "peoples"
        );
    };
}

#[macro_export]
macro_rules! ciboulette_query_test_relationship_many_to_one {
    ($transform_function:ident) => {
        ciboulette_query_test_routine!(
            relationships_many_to_one_sorting_by_child_full_path,
            $transform_function,
            "/comments/{}/relationships/author?sort=peoples.first-name",
            "comments"
        );
        ciboulette_query_test_routine!(
            relationships_many_to_one_sorting_by_child,
            $transform_function,
            "/comments/{}/relationships/author?sort=first-name",
            "comments"
        );
        ciboulette_query_test_routine!(
            relationships_many_to_one_sparse_child,
            $transform_function,
            "/comments/{}/relationships/author?fields[peoples]=first-name",
            "comments"
        );
        ciboulette_query_test_routine!(
            relationships_many_to_one_sort_and_sparse,
            $transform_function,
            "/comments/{}/relationships/author?fields[peoples]=first-name&sort=peoples.last-name",
            "comments"
        );
        ciboulette_query_test_routine!(
            relationships_many_to_one_include_root,
            $transform_function,
            "/comments/{}/relationships/author?include=articles",
            "comments"
        );
        ciboulette_query_test_routine!(
            relationships_many_to_one_include_and_sparse,
            $transform_function,
            "/comments/{}/relationships/author?include=articles&fields[peoples]=first-name",
            "comments"
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
		ciboulette_query_test_routine!(
			include_multiple_resources_with_sparsing,
			$transform_function,
			"/peoples/{}?include=peoples.articles,people-article&fields[peoples]=last-name&fields[articles]=title&fields[people-article]=article_id,people_id",
			"peoples"
		);
    };
}
