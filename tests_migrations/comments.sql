CREATE TABLE comments(
	id			UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
	body		TEXT,
	author		UUID NOT NULL REFERENCES peoples,
	articles	UUID NOT NULL REFERENCES articles
);
