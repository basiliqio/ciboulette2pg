use thiserror::Error;
pub type Ciboulette2SqlBufError =
    buf_redux::IntoInnerError<buf_redux::BufWriter<std::io::Cursor<Vec<u8>>>>;
/// An error throwable by this library
#[derive(Error, Debug)]
pub enum Ciboulette2SqlError {
    #[error("Cannot represent `{0}`")]
    BigDecimal(u128),
    #[error("Missing relationship `{1}` for type `{0}`")]
    MissingRelationship(String, String),
    #[error("The relationship for type `{0}` should have been singular")]
    RequiredSingleRelationship(String),
    #[error("The table `{0}` is unknown")]
    UnknownTable(String),
    #[error("A relationship value was empty for type `{0}`")]
    EmptyRelValue(String),
    #[error("A null char was found in a indentifier `${0}`")]
    NullCharIdent(String),
    #[error("Directly inserting/updating one-to-many or many-to-many relationships is forbidden")]
    ManyRelationshipDirectWrite,
    #[error("Updating main object cannot be done with relationships")]
    UpdatingMainObject,
    #[error("Multiple ids were provided for a one-to-one relationships")]
    MultiIdsForSingleRelationships,
    #[error("One of the needed relation for ordering couldn't be found : `{0}`")]
    MissingRelationForSorting(String),
    #[error("A non-ascii char was found in a indentifier `${0}`")]
    NonAsciiCharInIdent(String),
    #[error("Client provided `id`s are forbidden for inserts")]
    ProvidedIdOnInserts,
    #[error("The object attributes are required and missing")]
    MissingAttributes,
    #[error("Trying to sort type `{0}` using its one-to-many relationships to `{1}`")]
    SortingByMultiRel(String, String),
    #[error(transparent)]
    CibouletteError(#[from] ciboulette::CibouletteError),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    BufReaderInto(#[from] Ciboulette2SqlBufError),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("An unknown error occurred")]
    UnknownError,
}
