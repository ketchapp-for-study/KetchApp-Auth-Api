use actix_web::web;
pub mod register;
pub mod login;
pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(login::login_handler)
            .service(register::register_handler),
    );
}

