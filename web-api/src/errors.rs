use actix_web::{error::ResponseError, HttpResponse};
use thiserror::Error;
use database::errors::DatabaseError;

#[derive(Debug, Error, Serialize)]
pub enum ServiceError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("BadRequest: {0}")]
    BadRequest(String),

    #[error("Unable to connect to DB")]
    UnableToConnectToDb,

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError().json("Internal Server Error, Please try later"),
            ServiceError::UnableToConnectToDb => HttpResponse::InternalServerError().json("Unable to connect to DB, Please try later"),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized(ref message) =>  HttpResponse::Unauthorized().json(message),
        }
    }
}

impl From<DatabaseError> for ServiceError {
    fn from(db_error: DatabaseError) -> ServiceError {
        match db_error {
            DatabaseError::InternalServerError => ServiceError::InternalServerError,
            DatabaseError::UnableToConnectToDb => ServiceError::UnableToConnectToDb,
            DatabaseError::BadRequest(ref message) => ServiceError::BadRequest(String::from(message)),
            DatabaseError::Unauthorized(ref message) =>  ServiceError::Unauthorized(String::from(message)),
        }
    }
}

impl From<uuid::parser::ParseError> for ServiceError {
    fn from(_: uuid::parser::ParseError) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

pub type ServiceResult<V> = std::result::Result<V, ServiceError>;
