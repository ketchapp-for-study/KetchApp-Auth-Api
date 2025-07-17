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
            crate::models::user::User,
        )
    ),
    tags(
        (name = "authentication", description = "User authentication and management"),
    ),
    info(
        title = "Rust Authentication API",
        version = "0.1.0",
        description = "Comprehensive API documentation for the Rust Authentication service. This API provides endpoints for user registration, login, and potentially other user-related operations.", // More detailed description
        contact(
            name = "API Support",
            email = "support@example.com",
            url = "https://example.com/support"
        ),
        license(
            name = "MIT License",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    // Add external_docs for links to further documentation
    external_docs(
        description = "Find more info about the Rust Auth API",
        url = "https://github.com/"
    )
)]
pub struct ApiDoc;