use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::{Error as DBError};
use thiserror::Error;

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

impl From<uuid::parser::ParseError> for ServiceError {
    fn from(_: uuid::parser::ParseError) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        println!("DBError {0}", error);
        match error {
            DBError::DatabaseError(_kind, info) => {
                let message = info.details().unwrap_or_else(|| info.message()).to_string();
                println!("DBError message {0}", message);
                ServiceError::BadRequest(message)
            },
            DBError::NotFound =>  ServiceError::BadRequest(String::from("Resource not found in database")),
            _ => ServiceError::InternalServerError,
        }
    }
}

pub type ServiceResult<V> = std::result::Result<V, crate::errors::ServiceError>;
