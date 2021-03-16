INSERT INTO peoples ("first-name", "last-name", "favorite_color")
    VALUES ('AAAAAAAA', 'BBBBBBBBB', $1) RETURNING id;
