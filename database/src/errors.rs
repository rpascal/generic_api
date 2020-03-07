use diesel::result::{Error as DBError};
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
}

impl From<DBError> for DatabaseError {
    fn from(error: DBError) -> DatabaseError {
        println!("DBError {0}", error);
        match error {
            DBError::DatabaseError(_kind, info) => {
                let message = info.details().unwrap_or_else(|| info.message()).to_string();
                println!("DBError message {0}", message);
                DatabaseError::BadRequest(message)
            },
            DBError::NotFound =>  DatabaseError::BadRequest(String::from("Resource not found in database")),
            _ => DatabaseError::InternalServerError,
        }
    }
}

pub type DatabaseResult<V> = std::result::Result<V, crate::errors::DatabaseError>;
