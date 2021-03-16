INSERT INTO comments ("author", "articles", "body")
    VALUES ($1, $2, 'It was great !') RETURNING id;
