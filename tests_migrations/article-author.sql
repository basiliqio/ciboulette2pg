CREATE TABLE article_author(
	id				UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
	author_id		UUID NOT NULL REFERENCES peoples,
	article_id		UUID NOT NULL REFERENCES articles
);
