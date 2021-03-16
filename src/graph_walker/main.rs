use super::*;
use std::convert::TryFrom;

#[derive(Clone, Debug, Default, Getters)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresMain<'a> {
    pub insert_values: Vec<(&'a str, Ciboulette2SqlValue<'a>)>,
    pub single_relationships: Vec<&'a str>,
}

fn check_single_relationships<'a>(
    relationships: &'a BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>>,
    from_type_: &'a CibouletteResourceType,
    to_type_: &'a CibouletteResourceType,
    to_type_alias: &'a str,
    opt: &'a CibouletteRelationshipOneToOneOption,
) -> Result<Option<(&'a str, Ciboulette2SqlValue<'a>)>, Ciboulette2SqlError> {
    match relationships
        .get(to_type_alias)
        .map(|x| x.data())
        .unwrap_or(&CibouletteOptionalData::Null(false))
    {
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::One(rel_id)) => Ok(
            Some((opt.key().as_str(), Ciboulette2SqlValue::from(rel_id.id()))),
        ),
        CibouletteOptionalData::Object(CibouletteResourceIdentifierSelector::Many(_)) => {
            return Err(Ciboulette2SqlError::RequiredSingleRelationship(
                to_type_.name().to_string(),
            ));
        }
        CibouletteOptionalData::Null(x) if *x => {
            if !opt.optional() {
                return Err(Ciboulette2SqlError::MissingRelationship(
                    from_type_.name().to_string(),
                    to_type_.name().to_string(),
                ));
            }
            Ok(Some((opt.key().as_str(), Ciboulette2SqlValue::Text(None))))
        }
        CibouletteOptionalData::Null(_) => {
            if !opt.optional() {
                return Err(Ciboulette2SqlError::MissingRelationship(
                    from_type_.name().to_string(),
                    to_type_.name().to_string(),
                ));
            }
            Ok(None)
        }
    }
}

pub fn fill_attributes<'a>(
    args: &mut Vec<(&'a str, Ciboulette2SqlValue<'a>)>,
    obj: &'a Option<MessyJsonObjectValue<'a>>,
) -> Result<(), Ciboulette2SqlError> {
    if let Some(obj) = obj {
        for (k, v) in obj.iter() {
            if matches!(v, MessyJsonValue::Null(MessyJsonNullType::Absent, _)) {
                continue;
            }
            // Iterate over every attribute
            args.push((k, Ciboulette2SqlValue::try_from(v)?));
        }
    }
    Ok(())
}

pub fn extract_fields<'a>(
    store: &'a CibouletteStore,
    main_type: &'a CibouletteResourceType<'a>,
    attributes: &'a Option<MessyJsonObjectValue<'a>>,
    relationships: &'a BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>>,
    fails_on_many: bool,
) -> Result<Ciboulette2PostgresMain<'a>, Ciboulette2SqlError> {
    let mut res_val: Vec<(&'a str, Ciboulette2SqlValue<'a>)> = Vec::with_capacity(128);
    let mut res_rel: Vec<&'a str> = Vec::with_capacity(128);
    let main_type_index = store
        .get_type_index(main_type.name())
        .ok_or_else(|| CibouletteError::UnknownType(main_type.name().to_string()))?;

    fill_attributes(&mut res_val, &attributes)?;
    let mut walker = store
        .graph()
        .neighbors_directed(*main_type_index, petgraph::Direction::Outgoing)
        .detach(); // Create a graph walker
    while let Some((edge_index, node_index)) = walker.next(&store.graph()) {
        // For each connect edge outgoing from the original node
        let node_weight = store.graph().node_weight(node_index).unwrap(); // Get the node weight
        let alias: &String = main_type.get_alias(node_weight.name().as_str())?; // Get the alias translation of that resource

        if let CibouletteRelationshipOption::One(opt) =
            store.graph().edge_weight(edge_index).unwrap()
        {
            if let Some(v) =
                check_single_relationships(&relationships, &main_type, &node_weight, alias, opt)?
            {
                res_val.push(v); // Insert the relationship values
            }
            res_rel.push(alias.as_str());
        } else if fails_on_many && relationships.contains_key(alias.as_str()) {
            return Err(Ciboulette2SqlError::UpdatingManyRelationships);
        }
    }
    Ok(Ciboulette2PostgresMain {
        insert_values: res_val,
        single_relationships: res_rel,
    })
}

pub fn get_fields_single_rel<'a>(
    store: &'a CibouletteStore<'a>,
    main_type: &'a CibouletteResourceType<'a>,
) -> Result<Vec<&'a str>, Ciboulette2SqlError> {
    let mut res: Vec<&'a str> = Vec::with_capacity(128);
    let main_type_index = store
        .get_type_index(main_type.name())
        .ok_or_else(|| CibouletteError::UnknownType(main_type.name().to_string()))?;

    let mut walker = store
        .graph()
        .neighbors_directed(*main_type_index, petgraph::Direction::Outgoing)
        .detach(); // Create a graph walker
    while let Some((edge_index, node_index)) = walker.next(&store.graph()) {
        // For each connect edge outgoing from the original node
        let node_weight = store.graph().node_weight(node_index).unwrap(); // Get the node weight
        if let CibouletteRelationshipOption::One(_) = store.graph().edge_weight(edge_index).unwrap()
        {
            let alias: &String = main_type.get_alias(node_weight.name().as_str())?; // Get the alias translation of that resource
            res.push(alias.as_str());
        }
    }
    Ok(res)
}
