#[macro_use]
extern crate serde_derive;

use database;

mod errors;
mod routes;
mod configuration;

use configuration::cli_args::CliAndEnvArgs;

use actix_web::{App, HttpServer, middleware};
use crate::configuration::cli_and_env_arguments;

use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{dev, http, Result};
use actix_http::body::{Body, ResponseBody};
use actix_web::dev::ServiceResponse;

fn handle_bad_request<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<Body>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("application/json"),
    );
    let error_msg: String = match res.response().error() {
        Some(p) => format!("{:?}", p),
        None =>  String::from("Error internal")
    };
    let new_res: ServiceResponse<Body> = res.map_body(|_head, _body| {
        ResponseBody::Other(Body::Message(Box::new(error_msg)))
    });
    Ok(ErrorHandlerResponse::Response(new_res))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=debug,diesel=debug");

    env_logger::init();

    let args : CliAndEnvArgs = cli_and_env_arguments();

    let port = args.port;

    let pool = database::pool::establish_connection(&args.clone().database_url);

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(args.clone())
            .wrap(ErrorHandlers::new().handler(http::StatusCode::BAD_REQUEST, handle_bad_request))
            .wrap(middleware::Logger::default())
            .configure(routes::register_routes)
    })
        .bind(("localhost", port))
        .unwrap()
        .run();

    eprintln!("Listening on 0.0.0.0:{}", port);

    server.await
}
