pub mod model;
pub mod service;
pub mod validate_api_key;

use actix_web::{web, HttpResponse};
use actix_web::web::{Data, Json};
use crate::database::Pool;
use crate::errors::{ServiceError};
use crate::routes::user_endpoints::model::{UserEndpoint, InsertableUserEndpoint};

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/")
                    .wrap(validate_api_key::ValidateApiKey)
            // .guard(               web::route()
            //                           .guard(guard::Not(guard::Get()))
            //                           .to(HttpResponse::MethodNotAllowed),)
            .service(web::resource("/key").route(web::get().to(new_key)))
            .service(web::resource("/all").route(web::get().to(get_all)))
            .service(web::resource("/new_endpoint").to(new_endpoint))
            // .default_service(web::route().to(any))
                    // default
                    // .default_service(
                    //     // 404 for GET request
                    //     web::resource("")
                    //         .route(web::get().to(any))
                    //         // all requests that are not `GET`
                    //         .route(
                    //             web::route()
                    //                 // .guard(guard::Not(guard::Get()))
                    //                 .guard(  guard::fn_guard(
                    //                     |req| req.headers()
                    //                         .contains_key("content-type")))
                    //
                    //                 .to(nope_any),
                    //         ),
                    // )
            // .service()

    );
}

// async fn nope_any(pool: Data<Pool>) ->  Result<HttpResponse, ServiceError> {
//     // let response: Vec<UserEndpoint> = service::get_all::get_all(pool).unwrap();
//     Ok(HttpResponse::Ok().json( "nope_any Welcome to any"))
// }

// /// 404 handler
// async fn p404() -> Result<fs::NamedFile> {
//     Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
// }

// async fn nope_any(pool: Data<Pool>) ->  Result<HttpResponse, ServiceError> {
//     // let response: Vec<UserEndpoint> = service::get_all::get_all(pool).unwrap();
//     Ok(HttpResponse::Ok().json( "nope_any Welcome to any"))
// }
//
// async fn any(pool: Data<Pool>) ->  Result<HttpResponse, ServiceError> {
//     // let response: Vec<UserEndpoint> = service::get_all::get_all(pool).unwrap();
//     Ok(HttpResponse::Ok().json("Welcome to any"))
// }

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
