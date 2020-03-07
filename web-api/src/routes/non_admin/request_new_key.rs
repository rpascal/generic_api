use actix_web::{web, HttpResponse};
use actix_web::web::Data;
use database::Pool;
use crate::errors::ServiceResult;
use database::api_key::{model::ApiKey, request_new_key};

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/new_key")
            .service(web::resource("").route(web::get().to(new_key)))
    );
}

async fn new_key(pool: Data<Pool>) -> ServiceResult<HttpResponse>  {
    let response: ApiKey = request_new_key::execute(&pool).unwrap();
    Ok(HttpResponse::Ok().json(response))
}