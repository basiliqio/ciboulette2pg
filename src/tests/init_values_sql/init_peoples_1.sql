INSERT INTO peoples ("first-name", "last-name", "age", "gender", "favorite_color")
    VALUES ('Francis', 'Le Roy', '22', 'M', $1) RETURNING id;

