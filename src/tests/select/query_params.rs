use super::*;

ciboulette_query_test_multi!(test_select);
ciboulette_query_test_single!(test_select);

ciboulette_query_test_related!(test_select);

ciboulette_query_test_relationship_many_to_many!(test_select);

ciboulette_query_test_relationship_many_to_one!(test_select);
