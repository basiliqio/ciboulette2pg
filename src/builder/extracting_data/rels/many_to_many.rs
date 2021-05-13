use super::*;

/// Extract many-to-many/one-to-many relationships id from requests
pub(super) fn extract_data_from_relationship_details_many<'request>(
    acc: &mut Ciboulette2PgResourceInformations<'request>,
    relationship_data: &'request CibouletteRelationshipObject,
    rel_opt: Ciboulette2PgMultiRelationshipsType,
    rel_details: CibouletteResourceRelationshipDetails,
) {
    match relationship_data.data() {
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(rel_id)) => {
            acc.multi_relationships_mut().insert(
                rel_details.relation_alias().clone(),
                Ciboulette2PgMultiRelationships {
                    type_: rel_details.related_type().clone(),
                    rel_opt,
                    rel_details,
                    values: Some(Ciboulette2PgValue::from_id_selector(rel_id.id())),
                },
            );
        }
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::Many(rels_id)) => {
            acc.multi_relationships_mut().insert(
                rel_details.relation_alias().clone(),
                Ciboulette2PgMultiRelationships {
                    type_: rel_details.related_type().clone(),
                    rel_opt,
                    rel_details,
                    values: Some(
                        rels_id
                            .iter()
                            .map(|x| Ciboulette2PgValue::from_id_selector(x.id()))
                            .flatten()
                            .collect(),
                    ),
                },
            );
        }
        CibouletteOptionalData::Null(x) if *x => {
            acc.multi_relationships_mut().insert(
                rel_details.relation_alias().clone(),
                Ciboulette2PgMultiRelationships {
                    type_: rel_details.related_type().clone(),
                    rel_opt,
                    values: Some(Ciboulette2PgValue::from_id_type_selector(
                        rel_details.related_type().ids(),
                    )),
                    rel_details,
                },
            );
        }
        CibouletteOptionalData::Null(_) => {
            acc.multi_relationships_mut().insert(
                rel_details.relation_alias().clone(),
                Ciboulette2PgMultiRelationships {
                    type_: rel_details.related_type().clone(),
                    rel_opt,
                    rel_details,
                    values: None,
                },
            );
        }
    }
}
