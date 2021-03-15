pub async fn snapshot_table(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    name: &str,
    tables: &[&str],
) {
    let mut map = serde_json::Map::new();

    for table in tables.iter() {
        let snap: (Option<serde_json::Value>,) = sqlx::query_as(
            format!(
				"SELECT ARRAY_TO_JSON(ARRAY_AGG(result)) AS data FROM (SELECT * FROM \"{}\") result;",
				table
			)
            .as_str(),
        )
        .fetch_one(&mut *transaction)
        .await
        .expect("to result the database snapshot");
        match snap.0 {
            Some(x) => map.insert(table.to_string(), x),
            None => map.insert(table.to_string(), serde_json::Value::Null),
        };
    }

    insta::assert_json_snapshot!(name,
        serde_json::Value::Object(map),
    {
        ".*.*.id" => insta::dynamic_redaction(|value, _path| {
            assert_eq!(value
                .as_str()
                .unwrap()
                .chars()
                .filter(|&c| c == '-')
                .count(),
                4
            );
            "[uuid]"
        }),
        ".*.*.people_id" => insta::dynamic_redaction(|value, _path| {
            assert_eq!(value
                .as_str()
                .unwrap()
                .chars()
                .filter(|&c| c == '-')
                .count(),
                4
            );
            "[uuid]"
        }),
        ".*.*.article_id" => insta::dynamic_redaction(|value, _path| {
            assert_eq!(value
                .as_str()
                .unwrap()
                .chars()
                .filter(|&c| c == '-')
                .count(),
                4
            );
            "[uuid]"
        })
    });
}
