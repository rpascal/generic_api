use actix_web::{web, HttpResponse, HttpRequest, guard};
use actix_web::web::{Data, Json};
use crate::errors::{ServiceResult, ServiceError};
use uuid::Uuid;
use database::{Pool};
use database::get_request::{find_user_route, get_all_routes, new_endpoint};
use crate::routes::admin::validate_api_key::get_api_key_from_header_map;
use models::get_request::{GetRequest, BasicGetRequest};
use url::Url;
use std::collections::HashMap;
use actix_http::http::Uri;

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
    let query_params: HashMap<String, String> = get_query_params(_req.uri())?;

    let response: serde_json::Value = find_user_route::execute(path, query_params, header_api_key, &pool)?;
    Ok(HttpResponse::Ok().json(response))
}

async fn get_all_routes(_req: HttpRequest, pool: Data<Pool>) -> ServiceResult<HttpResponse> {
    let header_api_key: Uuid = get_api_key_from_header_map(_req.headers())?;
    let response: Vec<GetRequest> = get_all_routes::execute(header_api_key, &pool).unwrap();
    Ok(HttpResponse::Ok().json(response))
}

async fn new_endpoint(req: HttpRequest, _pool: Data<Pool>, _body: Json<BasicGetRequest>) -> ServiceResult<HttpResponse>  {
    let header_api_key: Uuid = get_api_key_from_header_map(req.headers())?;

    match new_endpoint::execute(header_api_key, &_pool, _body.into_inner()) {
        Ok(res) => {
            Ok(HttpResponse::Ok().json(res))
        },
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(e))
        }
    }
}

fn get_query_params(uri: &Uri) -> Result<HashMap<String, String>, ServiceError> {
    if let Some(path_and_query) = uri.path_and_query() {
        let mock_base_url = Url::parse("https://api.example.com");
        if let Ok(parsed_mock_url) = mock_base_url {
            let joined_url = parsed_mock_url.join(path_and_query.as_str());
            if let Ok(new_url) = joined_url {
                let query_pairs = new_url.query_pairs();
                let query_pairs_hash_query: HashMap<String, String> = query_pairs.into_owned().collect();
                return Ok(query_pairs_hash_query);
            }
        }
    }
    return Err(ServiceError::InternalServerError);
}