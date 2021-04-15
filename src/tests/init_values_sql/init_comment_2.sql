INSERT INTO comments ("author", "article", "body")
    VALUES ($1, $2, 'It was great !') RETURNING id;
