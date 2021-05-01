use super::*;

/// Extract attributes from the request and push them to an arguments vector
/// compatible with SQLx for later execution
pub fn attributes_to_sql_params<'store, 'request>(
    args: &mut Vec<(ArcStr, Ciboulette2PgValue<'request>)>,
    obj: &'request Option<MessyJsonObjectValue<'store>>,
) -> Result<(), Ciboulette2PgError> {
    if let Some(obj) = obj {
        // Iterate over every attribute
        for (k, v) in obj.iter() {
            // If the attribute is not there don't include it in the SQL query.
            // If it's forced to null, include it though
            if matches!(v, MessyJsonValue::Null(MessyJsonNullType::Absent, _)) {
                continue;
            }
            args.push((k.clone(), Ciboulette2PgValue::try_from(v)?));
        }
    }
    Ok(())
}
