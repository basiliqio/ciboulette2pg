use super::*;

fn gen_all_fields<'a>(
    type_: &'a ciboulette::CibouletteResourceType<'a>,
) -> Result<Vec<&'a str>, CibouletteError> {
    let mut res: Vec<&'a str> = type_
        .schema()
        .properties()
        .keys()
        .map(String::as_str)
        .collect();
    res.push("id");
    Ok(res)
}

pub fn extract_sparse<'a>(
    type_: &'a ciboulette::CibouletteResourceType<'a>,
    query: &'a CibouletteQueryParameters<'a>,
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
