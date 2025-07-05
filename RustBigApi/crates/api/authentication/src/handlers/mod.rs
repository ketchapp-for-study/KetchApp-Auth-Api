use actix_web::web;
pub mod register;
pub mod login;
pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(register::register_handler)
            .service(login::login_handler),
    );
}

