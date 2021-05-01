use super::*;
mod main;
mod rels;
mod structs;

pub(crate) use main::*;
pub(crate) use rels::*;
pub(crate) use structs::*;

/// Extract all the necessary data for build SQL queries from the attributes and
/// relationships
pub(crate) fn extract_data<'store, 'request>(
    store: &'store CibouletteStore,
    main_type: Arc<CibouletteResourceType>,
    attributes: &'request Option<MessyJsonObjectValue<'request>>,
    relationships: &'request BTreeMap<ArcStr, CibouletteRelationshipObject<'request>>,
    fails_on_many: bool,
) -> Result<Ciboulette2PgResourceInformations<'request>, Ciboulette2PgError>
where
    'store: 'request,
{
    let mut res = Ciboulette2PgResourceInformations::default();
    attributes_to_sql_params(&mut res.values, &attributes)?;

    for rel_alias in main_type.relationships().keys() {
        let rel_details = main_type.get_relationship_details(store, rel_alias)?;
        match relationships.get(rel_alias) {
            Some(rel_data) => extract_data_from_relationship_details(
                &mut res,
                &main_type,
                rel_details,
                rel_data,
                fails_on_many,
            )?,
            None => fill_relationships_without_data(&mut res, rel_details)?,
        }
    }
    Ok(res)
}

///
pub(crate) fn extract_data_no_body<'store, 'request>(
    store: &'store CibouletteStore,
    main_type: Arc<CibouletteResourceType>,
) -> Result<Ciboulette2PgResourceInformations<'request>, Ciboulette2PgError>
where
    'store: 'request,
{
    let mut res = Ciboulette2PgResourceInformations::default();
    for rel_alias in main_type.relationships().keys() {
        let rel_details = main_type.get_relationship_details(store, rel_alias)?;
        fill_relationships_without_data(&mut res, rel_details)?
    }
    Ok(res)
}
