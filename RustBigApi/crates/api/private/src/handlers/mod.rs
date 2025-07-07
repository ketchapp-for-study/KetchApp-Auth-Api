use actix_web::web;
use common::middlewares::auth_middleware::AuthMiddleware;
pub mod users;

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .wrap(AuthMiddleware)
            .service(users::get_users_handler),
    );
}