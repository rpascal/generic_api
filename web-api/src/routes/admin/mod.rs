use actix_web::web;

mod users_get_request;
mod validate_api_key;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(validate_api_key::ValidateApiKey)
            .configure(users_get_request::route)
    );
}