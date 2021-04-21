use super::*;

/// Extract attributes from the request and push them to an arguments vector
/// compatible with SQLx for later execution
pub fn fill_attributes<'store, 'request>(
    args: &mut Vec<(ArcStr, Ciboulette2SqlValue<'request>)>,
    obj: &'request Option<MessyJsonObjectValue<'store>>,
) -> Result<(), Ciboulette2SqlError> {
    if let Some(obj) = obj {
        for (k, v) in obj.iter() {
            if matches!(v, MessyJsonValue::Null(MessyJsonNullType::Absent, _)) {
                continue;
            }
            // Iterate over every attribute
            args.push((k.clone(), Ciboulette2SqlValue::try_from(v)?));
        }
    }
    Ok(())
}
