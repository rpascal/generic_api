pub mod model;
pub mod service;


use actix_web::{web, HttpResponse};
use actix_web::web::{Data, Json};
use crate::database::Pool;
use crate::errors::{ServiceError};
use crate::routes::user_endpoints::model::{UserEndpoint, InsertableUserEndpoint};

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .service(web::resource("/key").route(web::get().to(new_key)))
            .service(web::resource("/all").route(web::get().to(get_all)))

            // .service(web::resource("/new_endpoint").route(web::post().to(new_endpoint)))
            .service(web::resource("/new_endpoint").to(new_endpoint)
                // .route(web::post().to(new_endpoint))
                )

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

async fn new_endpoint(pool: Data<Pool>, body: Json<InsertableUserEndpoint>) -> Result<HttpResponse, ServiceError> {
    let response: UserEndpoint = service::new_endpoint::new_endpoint(pool, body).unwrap();
    Ok(HttpResponse::Ok().json(response))
}
