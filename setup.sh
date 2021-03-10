#!/bin/bash

export PGPASSWORD=postgres

# psql -U postgres -h localhost -d postgres <<EOF
#	CREATE EXTENSION "uuid-ossp";
# EOF

#psql -U postgres -h localhost -d postgres -a -f tests_migrations/favorite_color.sql \
#	&&
#psql -U postgres -h localhost -d postgres -a -f tests_migrations/peoples.sql \
#	&&
#psql -U postgres -h localhost -d postgres -a -f tests_migrations/articles.sql \
#	&&
#psql -U postgres -h localhost -d postgres -a -f tests_migrations/comments.sql \
#	&&
#psql -U postgres -h localhost -d postgres -a -f tests_migrations/article-author.sql

pg_restore --dbname=postgres --verbose db_backup.tar 
