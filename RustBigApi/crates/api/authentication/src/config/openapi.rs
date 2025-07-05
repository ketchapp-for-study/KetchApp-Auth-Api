use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::register::register_handler,
        crate::handlers::login::login_handler,
    ),
    components(
        schemas(
            crate::models::register::RegisterUser,
            crate::models::login::LoginUser,
            common::models::user::User,
        )
    ),
    tags(
        (name = "authentication", description = "User registration endpoint")
    ),
    info(
        title = "Rust Auth API",
        version = "0.1.0",
        description = "API Documentation"
    ))]
pub struct ApiDoc;
