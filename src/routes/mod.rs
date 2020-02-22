use actix_web::web;

pub mod hello;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    hello::route(cfg);
}