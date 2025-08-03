use actix_web::{
    cookie::{Cookie, SameSite},
    post, web, HttpResponse,
};
use argon2::{password_hash::PasswordHash, Argon2, PasswordVerifier};
use chrono::{Duration, Utc};
use validator::Validate;

use crate::{
    config::app_config::AppConfig,
    errors::{ErrorResponse, ServiceError},
    models::{auth_response_model::AuthResponse, claims::Claims, login::LoginUser, user::User},
    repositories::users_repo,
    DbPool,
};

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
            (status = 500, description = "Internal Server Error", body = ErrorResponse, example = json!({"code":500,"error":"Database Error","message":"Database connection failed"})),
            (status = 500, description = "JWT Key Error", body = ErrorResponse, example = json!({"code":500,"error":"JWT Key Error","message":"Errore lettura chiave privata"})),
            (status = 500, description = "JWT Generation Error", body = ErrorResponse, example = json!({"code":500,"error":"JWT Generation Error","message":"Errore generazione JWT"}))
        ),
        tag = "authentication"
    )]
#[post("/login")]
pub async fn login_handler(
    pool: web::Data<DbPool>,
    app_config: web::Data<AppConfig>,
    body: web::Json<LoginUser>,
) -> Result<HttpResponse, ServiceError> {
    // * * 1. Validazione dei dati di input ricevuti dal client
    let _validate = body
        .validate()
        .map_err(|e| ServiceError::ValidationError(format!("{:?}", e)))?;

    // * * 2. Recupero dell'utente dal database tramite username
    let user = users_repo::get_user_by_username(&pool, &body.username)
        .map_err(|_| ServiceError::Unauthorized("Invalid username or password".into()))?;

    // * 3. Parsing dell'hash della password salvata per il confronto
    let parsed_hash = PasswordHash::new(&user.password)
        .map_err(|_| ServiceError::JwtGenerationError("Failed to parse password hash".into()))?;

    // * 4. Verifica della password fornita rispetto all'hash salvato
    if Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(ServiceError::Unauthorized(
            "Invalid username or password".into(),
        ));
    }

    // * 5. Creazione dei claims per il JWT (contengono info utente e scadenza token)
    let now = Utc::now();
    let claims = Claims {
        sub: user.id.to_string(),
        exp: (now + Duration::seconds(app_config.jwt_exp_secs as i64)).timestamp() as usize,
        iat: now.timestamp() as usize,
        iss: app_config.jwt_issuer.clone(),
        aud: app_config.jwt_audience.clone(),
    };

    let private_key = std::fs::read("/app/private_key.pem")
        .map_err(|_| ServiceError::JwtGenerationError("Failed to read private key".into()))?;
    let token = claims
        .generate_jwt(&private_key)
        .map_err(|_| ServiceError::JwtGenerationError("Failed to generate JWT".into()))?;

    // * 7. Creazione di un cookie HTTP-only che contiene il token JWT
    let cookie = Cookie::build("auth_token", token.clone())
        .path("/")
        .http_only(true)
        .secure(app_config.is_production())
        .same_site(SameSite::Lax)
        .max_age(actix_web::cookie::time::Duration::seconds(
            Duration::days(1).num_seconds(),
        ))
        .finish();

    // * 8. Restituzione della risposta HTTP con il cookie e i dati dell'utente
    let user_res = AuthResponse {
        id: user.id,
        email: user.email,
        username: user.username,
        created_at: user.created_at,
        token,
    };

    Ok(HttpResponse::Ok().cookie(cookie).json(user_res))
}
