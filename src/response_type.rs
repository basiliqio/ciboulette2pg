use ciboulette::CibouletteResponseRequiredType;

#[derive(Clone, Debug, Copy)]
pub enum Ciboulette2PostgresResponseType {
    Object,
    Id,
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
