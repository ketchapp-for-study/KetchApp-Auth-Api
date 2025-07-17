use actix_web::{
    cookie::{Cookie, SameSite},
    post, web, HttpResponse,
};
use argon2::{
    password_hash::PasswordHash,
    Argon2, PasswordVerifier,
};
use chrono::{Duration, Utc};
use validator::Validate;

use crate::{
    config::app_config::AppConfig,
    errors::{ErrorResponse, ServiceError},
    models::{claims::Claims, login::LoginUser, user::User},
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
            (status = 500, description = "Internal Server Error", body = ErrorResponse)
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
    body.validate()
        .map_err(|e| ServiceError::ValidationError(format!("{:?}", e)))?;

    // * * 2. Recupero dell'utente dal database tramite username
    let user = users_repo::get_user_by_username(&pool, &body.username)
        .map_err(|_| ServiceError::InternalServerError)?;

    // * 3. Parsing dell'hash della password salvata per il confronto
    let parsed_hash =
        PasswordHash::new(&user.password).map_err(|_| ServiceError::InternalServerError)?;

    // * 4. Verifica della password fornita rispetto all'hash salvato
    if Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(ServiceError::unauthorized(
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

    // * 6. Generazione del token JWT firmato usando la chiave segreta
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET env variable");
    let token = claims
        .generate_jwt(&secret)
        .map_err(|_| ServiceError::InternalServerError)?;

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
    Ok(HttpResponse::Ok().cookie(cookie).json(user))
}
