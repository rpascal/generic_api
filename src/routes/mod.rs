use actix_web::web;

pub mod hello;
pub mod user_endpoints;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    hello::route(cfg);
    user_endpoints::route(cfg);
}