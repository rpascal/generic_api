use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error};
use futures::future::{ok, Either, Ready};
use actix_web::web::Data;
use uuid::Uuid;
use crate::errors::ServiceError;
use actix_http::http::HeaderMap;
use database::{Pool, api_key};

pub struct ValidateApiKey;

impl<S, B> Transform<S> for ValidateApiKey
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ValidateApiKeyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ValidateApiKeyMiddleware { service })
    }
}

pub struct ValidateApiKeyMiddleware<S> {
    service: S,
}

impl<S, B> Service for ValidateApiKeyMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let pool: Option<Data<Pool>> = req.app_data::<Pool>();

        match api_key_validation(&req, pool) {
            Ok(()) => {
                Either::Left(self.service.call(req))
            }
            Err(_e) => {
                Either::Right(ok(req.error_response(
                    _e,
                )))
            }
        }

    }
}

pub fn get_api_key_from_header_map(header_map: &HeaderMap) -> Result<Uuid, ServiceError> {
    if let Some(a) = header_map.get("api_key").ok_or_else(|| ServiceError::BadRequest(String::from("No header with api_key found")))?.to_str().ok() {
        return Ok(Uuid::parse_str(a)?);
    }
    return Err(ServiceError::BadRequest(String::from("No header with api_key found")));
}

fn api_key_validation(req: &ServiceRequest, pool: Option<Data<Pool>>) -> Result<(), ServiceError> {
    let header_api_key = get_api_key_from_header_map(req.headers())?;

    if let Some(pl) = pool {
        if let Ok(()) = api_key::api_key_validation::execute(header_api_key, &pl) {
            return Ok(());
        }
    }

    return Err(ServiceError::Unauthorized(format!("Bad api_key: {0}", header_api_key.to_string())));
}


