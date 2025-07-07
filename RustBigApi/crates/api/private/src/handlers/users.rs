use crate::DbPool;
use actix_web::{get, web, HttpResponse, HttpRequest};
use common::config::app_config::AppConfig;
use common::errors::{ErrorResponse, ServiceError};
use common::models::user::User;
use common::repositories::permissions_repo::get_permission_names_for_user;
use common::utils::extract_jwt_claims::extract_jwt_claims_from_request;
use diesel::prelude::*;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/api/users",
    responses(
        (status = 200, description = "User created", body = User),
        (status = 400, description = "Bad Request: invalid input", body = ErrorResponse),
        (status = 409, description = "Conflict: user already exists", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 422, description = "Unprocessable Entity: validation error", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    tag = "users")]
#[get("/users")]
pub async fn get_users_handler(
    pool: web::Data<DbPool>,
    app_config: web::Data<AppConfig>,
    req: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET env variable must be set");
    let claims = extract_jwt_claims_from_request(&req, &secret)
        .map_err(|e| ServiceError::Unauthorized(e.to_string()))?;

    // Check if the user has the "test_perms" permission
    let user_uuid = Uuid::parse_str(&claims.sub)
        .map_err(|_| ServiceError::Unauthorized("Invalid user UUID in token".to_string()))?;
    let mut conn = pool.get().map_err(|_| ServiceError::InternalServerError("DB connection error".to_string()))?;
    let permissions = get_permission_names_for_user(&mut conn, user_uuid)
        .map_err(|_| ServiceError::InternalServerError("Failed to fetch permissions".to_string()))?;
    if !permissions.contains(&"test_perms".to_string()) {
        return Err(ServiceError::Forbidden("Missing required permission: test_perms".to_string()));
    }

    let users = User::get_all_users(&pool)
        .await
        .map_err(|e| ServiceError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(users))
}
