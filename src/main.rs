#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

mod database;
mod errors;
mod routes;
mod configuration;

use configuration::cli_args::CliAndEnvArgs;

use actix_web::{App, HttpServer};
use crate::configuration::cli_and_env_arguments;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args : CliAndEnvArgs = cli_and_env_arguments();

    let port = args.port;

    let pool = database::pool::establish_connection(args.clone());

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(args.clone())
            .configure(routes::register_routes)
    })
        .bind(("localhost", port))
        .unwrap()
        .run();

    eprintln!("Listening on 0.0.0.0:{}", port);

    server.await
}
