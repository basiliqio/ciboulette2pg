#!/bin/bash

export PGPASSWORD=postgres

psql -U postgres -h localhost -d postgres <<EOF
	CREATE EXTENSION "uuid-ossp";
EOF

psql -U postgres -h localhost -d postgres -a -f tests_migrations/V2__favorite_color.sql \
	&&
psql -U postgres -h localhost -d postgres -a -f tests_migrations/V3__peoples.sql \
	&&
psql -U postgres -h localhost -d postgres -a -f tests_migrations/V4__articles.sql \
	&&
psql -U postgres -h localhost -d postgres -a -f tests_migrations/V5__comments.sql \
	&&
psql -U postgres -h localhost -d postgres -a -f tests_migrations/V6__article_author.sql

