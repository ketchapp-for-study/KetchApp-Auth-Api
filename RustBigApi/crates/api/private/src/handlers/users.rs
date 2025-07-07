use crate::DbPool;
use actix_web::{get, web, HttpResponse, HttpRequest};
use common::config::app_config::AppConfig;
use common::errors::{ErrorResponse, ServiceError};
use common::models::user::User;
use common::repositories::permissions_repo::get_permission_names_for_user;
use common::utils::extract_jwt_claims::extract_jwt_claims_from_request;
use diesel::prelude::*;
use permission_macro::permission;
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
#[permission("view_users")]
pub async fn get_users_handler(
    pool: web::Data<DbPool>,
    app_config: web::Data<AppConfig>,
    req: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    // Step 1: Extract JWT claims using the utility function
    // This will validate the JWT and return the claims if valid
    let claims = extract_jwt_claims_from_request(&req)
        .map_err(|e| ServiceError::Unauthorized(e.to_string()))?
        .ok_or_else(|| ServiceError::Unauthorized("Missing or invalid token".to_string()))?;

    // Step 2: Parse the user UUID from the JWT claims (subfield)
    let user_uuid = Uuid::parse_str(&claims.sub)
        .map_err(|_| ServiceError::Unauthorized("Invalid user UUID in token".to_string()))?;

    // Step 3: Fetch all permissions for the user (via their groups)
    let mut conn = pool.get().map_err(|_| ServiceError::InternalServerError)?;
    let permissions = get_permission_names_for_user(&mut conn, user_uuid)
        .map_err(|_| ServiceError::InternalServerError)?;

    // Step 4: Check if the user has the required permission (from the macro)
    if !permissions.contains(&REQUIRED_PERMISSION.to_string()) {
        return Err(ServiceError::Forbidden(String::from("Missing required permission.")));
    }

    // Step 5: Fetch and return all users as JSON
    let users = common::repositories::users_repo::get_users(&pool)
        .map_err(|_| ServiceError::InternalServerError)?;

    // Step 6: Return the users as a JSON response
    Ok(HttpResponse::Ok().json(users))
}

#[utoipa::path(
    get,
    path = "/api/users/@me",
    responses(
        (status = 200, description = "User created", body = User),
        (status = 400, description = "Bad Request: invalid input", body = ErrorResponse),
        (status = 409, description = "Conflict: user already exists", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 422, description = "Unprocessable Entity: validation error", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    tag = "users")]
#[get("/users/@me")]
// #[permission("view_me")] * Maybe add This?
pub async fn get_me_handler(
    pool: web::Data<DbPool>,
    app_config: web::Data<AppConfig>,
    req: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    // Step 1: Extract JWT claims using the utility function
    // This will validate the JWT and return the claims if valid
    let claims = extract_jwt_claims_from_request(&req)
        .map_err(|e| ServiceError::Unauthorized(e.to_string()))?
        .ok_or_else(|| ServiceError::Unauthorized("Missing or invalid token".to_string()))?;

    // Step 2: Parse the user UUID from the JWT claims (subfield)
    let user_uuid = Uuid::parse_str(&claims.sub)
        .map_err(|_| ServiceError::Unauthorized("Invalid user UUID in token".to_string()))?;

    // Step 3: Fetch all permissions for the user (via their groups)
    let mut conn = pool.get().map_err(|_| ServiceError::InternalServerError)?;

    // ! Da Aggiungere solo se metto #[permission("view_me")]
    // let permissions = get_permission_names_for_user(&mut conn, user_uuid)
    //     .map_err(|_| ServiceError::InternalServerError)?;
    //
    // // Step 4: Check if the user has the required permission (from the macro)
    // if !permissions.contains(&REQUIRED_PERMISSION.to_string()) {
    //     return Err(ServiceError::Forbidden(String::from("Missing required permission.")));
    // }

    // Step 5: Fetch and return all users as JSON
    let user = common::repositories::users_repo::get_user(&pool, user_uuid)
        .map_err(|_| ServiceError::InternalServerError)?;

    // Step 6: Return the users as a JSON response
    Ok(HttpResponse::Ok().json(user))
}

