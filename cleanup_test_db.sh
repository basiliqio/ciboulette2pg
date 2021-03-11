#!/bin/bash
psql --csv -U postgres -h localhost -d postgres -c "SELECT datname FROM pg_database WHERE datistemplate = false;" | grep -e 'basiliq_test_*' | parallel -q psql -e -U postgres -h localhost -d postgres -c 'DROP DATABASE "{}";';
