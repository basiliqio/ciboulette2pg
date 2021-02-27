use thiserror::Error;

#[derive(Error, Debug)]
pub enum Ciboulette2SqlError {
    // #[error("The json:api type `{0}` is unknown.")]
    // Mi(String),
    #[error("Cannot represent `{0}`")]
    BigDecimal(u128),
    #[error("Missing relationship `{1}` for type `{0}`")]
    MissingRelationship(String, String),
    #[error("The relationship for type `{0}` should have been singular")]
    RequiredSingleRelationship(String),
    #[error(transparent)]
    CibouletteError(#[from] ciboulette::CibouletteError),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    BufReaderInto(
        #[from] buf_redux::IntoInnerError<buf_redux::BufWriter<std::io::Cursor<Vec<u8>>>>,
    ),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
}
