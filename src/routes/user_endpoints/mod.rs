pub mod model;
pub mod service;
pub mod validate_api_key;

use actix_web::{web, HttpResponse, HttpRequest, guard};
use actix_web::web::{Data, Json};
use crate::database::Pool;
use crate::errors::ServiceError;
use crate::routes::user_endpoints::model::{ApiKey, BasicGetRequest, GetRequest};
use crate::routes::user_endpoints::validate_api_key::get_api_key_from_header_map;
use uuid::Uuid;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/new_key")
        .service(web::resource("").route(web::get().to(new_key)))
    );
    cfg.service(
        web::scope("/")
                    .guard(guard::Get())
                    .wrap(validate_api_key::ValidateApiKey)
                    .service(web::resource("/all_routes").to(get_all_routes))
                    .service(web::resource("/new_endpoint").to(new_endpoint))
                    .default_service(web::route().to(default_route))
    );
}

async fn default_route(_req: HttpRequest, pool: Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let path: &str = _req.path();
    let header_api_key: Uuid = get_api_key_from_header_map(_req.headers())?;
    let response: serde_json::Value = service::find_user_route::find_user_route(path, header_api_key, pool)?;
    Ok(HttpResponse::Ok().json(response))
}

async fn get_all_routes(_req: HttpRequest, pool: Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let header_api_key: Uuid = get_api_key_from_header_map(_req.headers())?;
    let response: Vec<GetRequest> = service::get_all_routes::get_all_routes(header_api_key, pool).unwrap();
    Ok(HttpResponse::Ok().json(response))
}

async fn new_key(pool: Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let response: ApiKey = service::request_new_key::request_new_key(pool).unwrap();
    Ok(HttpResponse::Ok().json(response))
}

async fn new_endpoint(_req: HttpRequest, _pool: Data<Pool>, _body: Json<BasicGetRequest>) -> Result<HttpResponse, ServiceError> {
    let header_api_key: Uuid = get_api_key_from_header_map(_req.headers())?;
    match service::new_endpoint::new_endpoint(header_api_key, _pool, _body.into_inner()) {
        Ok(res) => {
            Ok(HttpResponse::Ok().json(res))
        },
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(e))
        }
    }
}
