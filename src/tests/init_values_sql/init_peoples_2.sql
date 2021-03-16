INSERT INTO peoples ("first-name", "last-name", "age", "gender", "twitter", "favorite_color")
    VALUES ('Somebody', 'Wuhu', '34', 'F', '@randomhandle', $1) RETURNING id;
