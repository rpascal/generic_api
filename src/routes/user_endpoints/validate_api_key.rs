use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Either, Ready};
use actix_web::web::Data;
use crate::database::{Pool, db_connection};
use crate::routes::user_endpoints::model::UserEndpoint;
use uuid::Uuid;
use crate::errors::ServiceError;
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};

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
        let pool = req.app_data::<Pool>();

        match api_key_validation(&req, pool) {
            Ok(()) => {
                Either::Left(self.service.call(req))
            }
            Err(_e) => {
                Either::Right(ok(req.into_response(
                    HttpResponse::Unauthorized().finish().into_body(),
                )))
            }
        }
    }
}

fn get_api_key(req: &ServiceRequest) -> Result<Uuid, ServiceError> {
    if let Some(a) = req.headers().get("apikey").ok_or_else(|| ServiceError::InternalServerError)?.to_str().ok() {
        return Ok(Uuid::parse_str(a)?);
    }
    return Err(ServiceError::InternalServerError);
}

fn api_key_validation(req: &ServiceRequest, pool: Option<Data<Pool>>) -> Result<(), ServiceError> {
    if let Some(p) = pool {
        let conn = &db_connection(&p)?;
        use crate::database::schema::user_endpoints::{table, key};

        let api_key = get_api_key(&req)?;

        println!("Found api_key {}", api_key);
        let results = table.filter(key.eq(api_key))
            .limit(1)
            .load::<UserEndpoint>(conn)
            .expect("Error loading posts");

        if results.len() > 0 {
            return Ok(());
        }
    }
    return Err(ServiceError::InternalServerError);
}
