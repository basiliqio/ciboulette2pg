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
    #[error("The table `{0}` is unknown")]
    UnknownTable(String),
    #[error("A relationship value was empty for type `{0}`")]
    EmptyRelValue(String),
    #[error("A null char was found in a indentifier `${0}`")]
    NullCharIdent(String),
    #[error("Updating relationships cannot be done with main object")]
    UpdatingRelationships,
    #[error("Updating one-to-many or many-to-many relationships is forbidden")]
    UpdatingManyRelationships,
    #[error("Updating main object cannot be done with relationships")]
    UpdatingMainObject,
    #[error("Multiple ids were provided for a one-to-one relationships")]
    MultiIdsForSingleRelationships,
    #[error("One-to-many relationship can't be deleted in bulk")]
    BulkRelationshipDelete,
    #[error("Non optional relationship `{1}` for type `{0}`")]
    RequiredRelationship(String, String),
    #[error("A non-ascii char was found in a indentifier `${0}`")]
    NonAsciiCharInIdent(String),
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
    #[error("An unknown error occurred")]
    UnkownError,
}
