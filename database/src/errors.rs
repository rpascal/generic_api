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

impl From<arangors::Error> for DatabaseError {
    fn from(err: arangors::Error) -> Self {
        let error_msg: String = format!("{:?}", err);
        DatabaseError::BadRequest(error_msg)    }
}

impl From<failure::Error> for DatabaseError {
    fn from(err: failure::Error) -> Self {
        use failure::AsFail;
        let error_msg: String = format!("{:?}", err.as_fail());
        DatabaseError::BadRequest(error_msg)
    }
}

pub type DatabaseResult<V> = std::result::Result<V, crate::errors::DatabaseError>;
