use crate::models::claims::Claims;
use crate::models::login::LoginUser;
use crate::DbPool;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{post, web, HttpResponse};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::PasswordHash;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use common::config::app_config::AppConfig;
use common::errors::{ErrorResponse, ServiceError};
use common::models::user::User;
use common::repositories::users_repo;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/api/login",
    request_body = LoginUser,
    responses(
        (status = 200, description = "User created", body = User),
        (status = 400, description = "Bad Request: invalid input", body = ErrorResponse),
        (status = 409, description = "Conflict: user already exists", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 422, description = "Unprocessable Entity: validation error", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    tag = "authentication")]
#[post("/login")]
pub async fn login_handler(
    pool: web::Data<DbPool>,
    app_config: web::Data<AppConfig>,
    body: web::Json<LoginUser>,
) -> Result<HttpResponse, ServiceError> {
    // Step 1: Validazione input lato Rust
    if let Err(validation_errors) = body.validate() {
        return Err(ServiceError::ValidationError(format!(
            "{:?}",
            validation_errors
        )));
    }

    // Step 2: Hash password with Argon2
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|_| ServiceError::ValidationError("Password hashing failed".into()))?
        .to_string();

    // Step 3: Retrieve user from database
    let user = users_repo::get_user_by_username(&pool, &body.username)
        .map_err(|_| ServiceError::InternalServerError)?;

    // Convert the password hash string to a PasswordHash object
    let parsed_hash =
        PasswordHash::new(&user.password).map_err(|_| ServiceError::InternalServerError)?;

    // Step 4: Verify password
    if !argon2
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        return Err(ServiceError::Unauthorized(
            "Invalid username or password".into(),
        ));
    }

    // Step 5: Create JWT claims
    let claims = Claims {
        sub: user.id.to_string(),
        exp: (Utc::now() + Duration::seconds(app_config.jwt_exp_secs as i64)).timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
        iss: app_config.jwt_issuer.clone(),
        aud: app_config.jwt_audience.clone(),
        roles: Vec::new()
    };

    // Step 6: Generate JWT token
    // Use the JWT secret from the .env file
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET env variable not set");
    let token = claims
        .generate_jwt(&secret)
        .map_err(|_| ServiceError::InternalServerError)?;

    // Step 7: Create cookie with JWT token
    let cookie = Cookie::build("auth_token", token)
        .path("/")
        .http_only(true)
        .secure(app_config.is_production())
        .same_site(SameSite::Lax)
        .max_age(actix_web::cookie::time::Duration::seconds(Duration::days(1).num_seconds()))
        .finish();

    // Step 8: Return response with user data and cookie
    Ok(HttpResponse::Ok().cookie(cookie).json(user))
}
