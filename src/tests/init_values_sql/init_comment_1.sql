INSERT INTO comments ("author", "articles", "body")
    VALUES ($1, $2, 'Wasnt convinced...') RETURNING id;
