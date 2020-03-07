use actix_web::web;

pub mod non_admin;
pub mod admin;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    non_admin::register_routes(cfg);
    admin::register_routes(cfg);
}