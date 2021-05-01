use ciboulette::CibouletteResponseRequiredType;

/// The response type of a resource
#[derive(Clone, Debug, Copy)]
pub enum Ciboulette2PgResponseType {
    /// Return the whole object, with its included attributes
    Object,
    /// Return only the identifier of the object
    Id,
    /// Don't return the object
    None,
}

impl From<CibouletteResponseRequiredType> for Ciboulette2PgResponseType {
    fn from(other: CibouletteResponseRequiredType) -> Ciboulette2PgResponseType {
        match other {
            CibouletteResponseRequiredType::Object(_) => Ciboulette2PgResponseType::Object,
            CibouletteResponseRequiredType::Id(_) => Ciboulette2PgResponseType::Id,
            CibouletteResponseRequiredType::None => Ciboulette2PgResponseType::None,
        }
    }
}
