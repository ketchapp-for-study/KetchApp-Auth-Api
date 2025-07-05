use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
    ),
    components(
        schemas(
        )
    ),
    tags(
        (name = "users", description = "User registration endpoint")
    ),
    info(
        title = "Rust Private API",
        version = "0.1.0",
        description = "API Documentation"
    ))]
pub struct ApiDoc;
