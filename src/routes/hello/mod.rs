use actix_web::{Responder, HttpRequest, web};
use actix_web::web::Data;
use crate::database::Pool;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/hello")
            .service(web::resource("").route(web::get().to(hello)))
    );
}

async fn hello() -> impl Responder {
    format!("{}", " Hello!")
}