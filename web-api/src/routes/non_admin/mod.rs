use actix_web::web;

mod hello;
mod request_new_key;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    hello::route(cfg);
    request_new_key::route(cfg);
}