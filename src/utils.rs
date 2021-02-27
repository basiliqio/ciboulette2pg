use super::*;

fn gen_all_fields<'a>(
    type_: &'a ciboulette::CibouletteResourceType,
) -> Result<Vec<&'a str>, CibouletteError> {
    if let MessyJson::Obj(obj) = type_.schema() {
        let mut res: Vec<&'a str> = obj.properties().keys().map(String::as_str).collect();
        res.push("id");
        Ok(res)
    } else {
        Err(CibouletteError::AttributesIsNotAnObject)
    }
}

pub fn extract_sparse<'a>(
    type_: &'a ciboulette::CibouletteResourceType,
    query: &'a CibouletteQueryParameters<'_>,
) -> Result<Vec<&'a str>, CibouletteError> {
    let sparse = query.sparse().get(type_);
    match sparse {
        None => gen_all_fields(type_),
        Some(x) if x.len() == 0 => gen_all_fields(type_),
        Some(x) => {
            let mut res: Vec<&'a str> = x.iter().map(Cow::as_ref).collect();
            res.push("id");
            Ok(res)
        }
    }
}
