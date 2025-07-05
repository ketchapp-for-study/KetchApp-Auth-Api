use actix_web::web;
pub mod users;

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // .service(register::register_handler),
    );
}

