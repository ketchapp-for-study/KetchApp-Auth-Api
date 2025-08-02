use crate::{
    config::app_config::AppConfig,
    errors::{ErrorResponse, ServiceError},
    models::{
        auth_response_model::AuthResponse, claims::Claims, register::RegisterUser, user::User,
    },
    repositories::{establish_connection, users_repo},
    DbPool,
};
use actix_web::{
    cookie::{Cookie, SameSite},
    post, web, HttpResponse,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use rand::rngs::OsRng;
use tracing::error;
use validator::Validate;

#[utoipa::path(
        post,
        path = "/api/register",
        request_body = RegisterUser,
        responses(
            (status = 200, description = "User created", body = AuthResponse),
            (status = 400, description = "Bad Request: invalid input", body = ErrorResponse),
            (status = 409, description = "Conflict: user already exists", body = ErrorResponse),
            (status = 404, description = "Not Found", body = ErrorResponse),
            (status = 422, description = "Unprocessable Entity: validation error", body = ErrorResponse),
            (status = 500, description = "Internal Server Error", body = ErrorResponse),
            (status = 500, description = "Database Error", body = ErrorResponse, example = json!({"code":500,"error":"Database Error","message":"Database connection failed"})),
            (status = 500, description = "JWT Key Error", body = ErrorResponse, example = json!({"code":500,"error":"JWT Key Error","message":"Errore lettura chiave privata"})),
            (status = 500, description = "JWT Generation Error", body = ErrorResponse, example = json!({"code":500,"error":"JWT Generation Error","message":"Errore generazione JWT"}))
        ),
        tag = "authentication"
    )]
#[post("/register")]
pub async fn register_handler(
    pool: web::Data<DbPool>,
    app_config: web::Data<AppConfig>,
    body: web::Json<RegisterUser>,
) -> Result<HttpResponse, ServiceError> {
    // Validazione input
    if let Err(validation_errors) = body.validate() {
        error!(
            "Validation error for new user {}: {:?}",
            body.username, validation_errors
        );
        return Err(ServiceError::ValidationError(format!(
            "{:?}",
            validation_errors
        )));
    }

    if users_repo::user_exists_by_username_or_email(&pool, &body.username, &body.email).map_err(
        |err| {
            error!(
                "Database error checking user existence for {}: {:?}",
                body.username, err
            );
            ServiceError::DatabaseError(err)
        },
    )? {
        error!(
            "Conflict: Username '{}' or email '{}' already in use.",
            body.username, body.email
        );
        return Err(ServiceError::Conflict("Username o email già in uso".into()));
    }

    let private_key = std::fs::read("./private_key.pem").map_err(|err| {
        error!("JWT Key Error: Errore lettura chiave privata: {:?}", err);
        ServiceError::JwtKeyError(format!("Errore lettura chiave privata: {:?}", err))
    })?;

    let mut rng = OsRng;
    let salt = SaltString::generate(&mut rng);
    let password_hash = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            error!(
                "Password hashing failed for user {}: {:?}",
                body.username, e
            );
            ServiceError::ValidationError("Hashing della password fallito".into())
        })?
        .to_string();

    let new_user = users_repo::NewUser {
        username: body.username.clone(),
        email: body.email.clone(),
        password: password_hash,
    };

    let (user, token) = web::block({
        let pool = pool.clone();
        let private_key = private_key.clone();
        let app_config = app_config.clone();
        let new_user = new_user;

        move || -> Result<(User, String), ServiceError> {
            let mut conn = establish_connection(&pool).map_err(|e| {
                error!(
                    "Database connection error in blocking thread for new user {}: {:?}",
                    new_user.username, e
                );
                ServiceError::DatabaseError(e)
            })?;

            conn.transaction::<(User, String), diesel::result::Error, _>(|conn| {
                let user = users_repo::create_user_with_connection(conn, new_user.clone())?;

                let now = Utc::now();
                let claims = Claims {
                    sub: user.id.to_string(),
                    exp: (now + Duration::seconds(app_config.jwt_exp_secs as i64)).timestamp()
                        as usize,
                    iat: now.timestamp() as usize,
                    iss: app_config.jwt_issuer.clone(),
                    aud: app_config.jwt_audience.clone(),
                };

                let token = claims.generate_jwt(&private_key).map_err(|e| {
                    error!(
                        "JWT generation failed within transaction for user {}: {:?}",
                        user.username, e
                    );
                    diesel::result::Error::RollbackTransaction
                })?;

                Ok((user, token))
            })
            .map_err(|e| {
                error!(
                    "Database transaction failed for user {}: {:?}",
                    new_user.username, e
                );
                ServiceError::DatabaseError(e)
            })
        }
    })
    .await
    .map_err(|e| {
        error!(
            "Blocking operation failed for user {}: {:?}",
            body.username, e
        );
        ServiceError::JwtGenerationError(format!("Errore generazione JWT: {}", e))
    })??;

    let cookie = Cookie::build("auth_token", token.clone())
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(true)
        .finish();

    let auth_response = AuthResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        created_at: user.created_at,
        token,
    };

    Ok(HttpResponse::Ok().cookie(cookie).json(auth_response))
}
