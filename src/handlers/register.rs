use actix_web::{
    cookie::{Cookie, SameSite},
    post, web, HttpResponse,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::{Duration, Utc};
use rand::rngs::OsRng;
use validator::Validate;

use crate::{
    config::app_config::AppConfig,
    errors::{ErrorResponse, ServiceError},
    models::{claims::Claims, register::RegisterUser, user::User},
    repositories::users_repo,
    DbPool,
};

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
        tag = "authentication"
    )]
// ... (resto degli import e del codice invariato)
#[post("/register")]
pub async fn register_handler(
    pool: web::Data<DbPool>,
    app_config: web::Data<AppConfig>,
    body: web::Json<RegisterUser>,
) -> Result<HttpResponse, ServiceError> {
    body.validate()
        .map_err(|e| ServiceError::ValidationError(format!("{:?}", e)))?;

    if users_repo::user_exists_by_username_or_email(&pool, &body.username, &body.email)
        .map_err(|err| ServiceError::DatabaseError(err))?
    {
        return Err(ServiceError::Conflict("Username o email già in uso".into()));
    }

    let mut rng = OsRng;
    let salt = SaltString::generate(&mut rng);
    let password_hash = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|_| ServiceError::ValidationError("Hashing della password fallito".into()))?
        .to_string();

    let new_user = users_repo::NewUser {
        username: body.username.clone(),
        email: body.email.clone(),
        password: password_hash,
    };

    let user = web::block({
        let pool = pool.clone();
        move || users_repo::new_user(&pool, new_user)
    })
    .await
    .map_err(|_| ServiceError::InternalServerError)?
    .map_err(|_| ServiceError::DatabaseError(diesel::result::Error::RollbackTransaction))?;

    let now = Utc::now();
    let claims = Claims {
        sub: user.id.to_string(),
        exp: (now + Duration::seconds(app_config.jwt_exp_secs as i64)).timestamp() as usize,
        iat: now.timestamp() as usize,
        iss: app_config.jwt_issuer.clone(),
        aud: app_config.jwt_audience.clone(),
    };

    let private_key = std::fs::read("/app/private_key.pem").map_err(|err| {
        ServiceError::JwtKeyError(format!("Errore lettura chiave privata: {:?}", err))
    })?;
    let token = claims.generate_jwt(&private_key).map_err(|err| {
        ServiceError::JwtGenerationError(format!("Errore generazione JWT: {:?}", err))
    })?;

    let cookie = Cookie::build("auth_token", token.clone())
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(true)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).body(token))
}
