pub mod model;
pub mod service;


use actix_web::{web, HttpResponse};
use actix_web::web::Data;
use crate::database::Pool;
use crate::errors::{ServiceError};
use crate::routes::user_endpoints::model::{UserEndpoint};

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .service(web::resource("/key").route(web::get().to(new_key)))
            .service(web::resource("/all").route(web::get().to(get_all)))
    );
}

async fn get_all(pool: Data<Pool>) ->  Result<HttpResponse, ServiceError> {
    let response: Vec<UserEndpoint> = service::get_all::get_all(pool).unwrap();
    Ok(HttpResponse::Ok().json(response))
}

async fn new_key(pool: Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let response: UserEndpoint = service::request_new_key::request_new_key(pool).unwrap();
    Ok(HttpResponse::Ok().json(response))
}
