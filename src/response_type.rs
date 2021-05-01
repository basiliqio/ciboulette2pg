use ciboulette::CibouletteResponseRequiredType;

/// The response type of a resource
#[derive(Clone, Debug, Copy)]
pub enum Ciboulette2PostgresResponseType {
    /// Return the whole object, with its included attributes
    Object,
    /// Return only the identifier of the object
    Id,
    /// Don't return the object
    None,
}

impl From<CibouletteResponseRequiredType> for Ciboulette2PostgresResponseType {
    fn from(other: CibouletteResponseRequiredType) -> Ciboulette2PostgresResponseType {
        match other {
            CibouletteResponseRequiredType::Object(_) => Ciboulette2PostgresResponseType::Object,
            CibouletteResponseRequiredType::Id(_) => Ciboulette2PostgresResponseType::Id,
            CibouletteResponseRequiredType::None => Ciboulette2PostgresResponseType::None,
        }
    }
}
