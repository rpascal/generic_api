use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum DatabaseError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("BadRequest: {0}")]
    BadRequest(String),

    #[error("Unable to connect to DB")]
    UnableToConnectToDb,

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Serialization Error")]
    SerializationError,
}

impl From<serde_json::error::Error> for DatabaseError {
    fn from(_: serde_json::error::Error) -> Self {
       DatabaseError::SerializationError
    }
}

pub type DatabaseResult<V> = std::result::Result<V, crate::errors::DatabaseError>;
