INSERT INTO comments ("author", "article", "body")
    VALUES ($1, $2, 'Wasnt convinced...') RETURNING id;
