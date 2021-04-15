use super::*;
pub async fn snapshot_table(
    pool: &mut sqlx::PgPool,
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
        .fetch_one(&mut pool.acquire().await.unwrap())
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
        ".*.*.id" => insta::dynamic_redaction(check_uuid),
        ".*.*.related_id" => insta::dynamic_redaction(check_uuid),
        ".*.*.people_id" => insta::dynamic_redaction(check_uuid),
        ".*.*.article_id" => insta::dynamic_redaction(check_uuid),
        ".*.*.articles" => insta::dynamic_redaction(check_uuid),
        ".*.*.author" => insta::dynamic_redaction(check_uuid),
        ".*.*.favorite_color" => insta::dynamic_redaction(|value, _path| {
            match value
            .as_str()
            {
                Some(x) => {
                    assert_eq!(x
                        .chars()
                        .filter(|&c| c == '-')
                        .count(),
                        4
                    );
                    "[favorite_color_uuid]"
                },
                    None => "[favorite_color_null]"
            }
        })
    });
}
