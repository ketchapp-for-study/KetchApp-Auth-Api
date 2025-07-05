use crate::models::claims::Claims;
use crate::models::register::RegisterUser;
use crate::DbPool;
use actix_web::{post, web, HttpResponse};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use chrono::{Duration, Utc};
use common::errors::{ErrorResponse, ServiceError};
use common::models::user::User;
use common::repositories::users_repo;
use validator::Validate;
use common::config::app_config::AppConfig;
use actix_web::cookie::{Cookie, SameSite};

#[utoipa::path(
    post,
    path = "/api/register",
    request_body = RegisterUser,
    responses(
        (status = 200, description = "User created", body = User),
        (status = 400, description = "Bad Request: invalid input", body = ErrorResponse),
        (status = 409, description = "Conflict: user already exists", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 422, description = "Unprocessable Entity: validation error", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    ),
    tag = "authentication")]
#[post("/register")]
pub async fn register_handler(
    pool: web::Data<DbPool>,
    app_config: web::Data<AppConfig>,
    body: web::Json<RegisterUser>,
) -> Result<HttpResponse, ServiceError> {
    // Step 1: Validazione input lato Rust
    if let Err(validation_errors) = body.validate() {
        return Err(ServiceError::ValidationError(format!(
            "{:?}",
            validation_errors
        )));
    }

    // Step 1.1: Check if username or email already exists
    if users_repo::user_exists_by_username_or_email(&pool, &body.username, &body.email)
        .map_err(|_| ServiceError::InternalServerError)?
    {
        return Err(ServiceError::Conflict("Username or email already in use".into()));
    }

    // Step 2: Hash password with Argon2
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|_| ServiceError::ValidationError("Password hashing failed".into()))?
        .to_string();

    // Step 3: Conversione RegisterUser -> NewUser (usando la password hashata)
    let new_user = users_repo::NewUser {
        username: body.username.clone(),
        email: body.email.clone(),
        password: password_hash,
    };
    let pool_for_block = pool.clone();

    // Step 4: Inserimento utente nel DB
    let user = match web::block(move || users_repo::new_user(&pool_for_block, new_user)).await {
        Ok(Ok(user)) => user,
        Ok(Err(_)) => return Err(ServiceError::DatabaseError(diesel::result::Error::RollbackTransaction)),
        Err(_) => return Err(ServiceError::InternalServerError),
    };

    // Step 5: Fetch user roles from DB
    let roles = match users_repo::get_user_roles(user.id, &pool) {
        Ok(roles) => roles,
        Err(_) => return Err(ServiceError::NotFound("User roles not found".into())),
    };

    // Step 6: Generate claims and JWT
    let now = Utc::now();
    let claims = Claims {
        sub: user.id.to_string(),
        exp: (now + Duration::seconds(app_config.jwt_exp_secs as i64)).timestamp() as usize,
        iat: now.timestamp() as usize,
        iss: app_config.jwt_issuer.clone(),
        aud: app_config.jwt_audience.clone(),
        roles: Vec::new(),
    };
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET env variable not set");
    let token = match jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    ) {
        Ok(token) => token,
        Err(_) => return Err(ServiceError::InternalServerError),
    };

    // Step 7: Response with JWT in cookie
    let cookie = Cookie::build("auth_token", token.clone())
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(true)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .body(token))
}
