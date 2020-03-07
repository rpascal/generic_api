use actix_web::{Responder, web};

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/hello")
            .service(web::resource("").route(web::get().to(hello)))
    );
}

async fn hello() -> impl Responder {
    format!("{}", " Hello!")
}