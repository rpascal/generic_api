use actix_web::{web, HttpResponse, HttpRequest, guard};
use actix_web::web::{Data, Json};
use crate::errors::ServiceResult;
use uuid::Uuid;
use database::{Pool};
use database::get_request::{find_user_route, get_all_routes, new_endpoint};
use crate::routes::admin::validate_api_key::get_api_key_from_header_map;
use models::get_request::{GetRequest, BasicGetRequest};

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/")
            .guard(guard::Get())
            .service(web::resource("/all_routes").to(get_all_routes))
            .service(web::resource("/new_endpoint").to(new_endpoint))
            .default_service(web::route().to(default_route))
    );
}

async fn default_route(_req: HttpRequest, pool: Data<Pool>) -> ServiceResult<HttpResponse> {
    let path: &str = _req.path();
    let header_api_key: Uuid = get_api_key_from_header_map(_req.headers())?;
    let response: serde_json::Value = find_user_route::execute(path, header_api_key, &pool)?;
    Ok(HttpResponse::Ok().json(response))
}

async fn get_all_routes(_req: HttpRequest, pool: Data<Pool>) -> ServiceResult<HttpResponse> {
    let header_api_key: Uuid = get_api_key_from_header_map(_req.headers())?;
    let response: Vec<GetRequest> = get_all_routes::execute(header_api_key, &pool).unwrap();
    Ok(HttpResponse::Ok().json(response))
}

async fn new_endpoint(_req: HttpRequest, _pool: Data<Pool>, _body: Json<BasicGetRequest>) -> ServiceResult<HttpResponse>  {
    let header_api_key: Uuid = get_api_key_from_header_map(_req.headers())?;
    match new_endpoint::execute(header_api_key, &_pool, _body.into_inner()) {
        Ok(res) => {
            Ok(HttpResponse::Ok().json(res))
        },
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(e))
        }
    }
}
