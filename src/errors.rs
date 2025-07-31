use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Blocking operation failed")]
    BlockingError,
    #[error("Database Error: {0}")]
    DatabaseError(#[from] DieselError),
    #[error("Database Pool Error: {0}")]
    PoolError(#[from] r2d2::Error),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("JWT Key Error: {0}")]
    JwtKeyError(String),
    #[error("JWT Generation Error: {0}")]
    JwtGenerationError(String),
}

impl ServiceError {
    pub fn unauthorized(message: String) -> Self {
        ServiceError::ValidationError(message)
    }
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub code: u16,
    pub error: String,
    pub message: String,
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ServiceError::NotFound(_) => StatusCode::NOT_FOUND,
            ServiceError::Conflict(_) => StatusCode::CONFLICT,
            ServiceError::Forbidden(_) => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let binding = self.to_string();
        let (error, message) = match self {
            ServiceError::ValidationError(msg) => ("Validation Error", msg.clone()),
            ServiceError::NotFound(msg) => ("Not Found", msg.clone()),
            ServiceError::Conflict(msg) => ("Conflict", msg.clone()),
            ServiceError::Forbidden(msg) => ("Forbidden", msg.clone()),
            _ => (binding.as_str(), self.to_string()),
        };
        let error_response = ErrorResponse {
            code: self.status_code().as_u16(),
            error: error.to_string(),
            message,
        };
        HttpResponse::build(self.status_code()).json(error_response)
    }
}

impl From<actix_web::error::BlockingError> for ServiceError {
    fn from(_: actix_web::error::BlockingError) -> Self {
        ServiceError::BlockingError
    }
}
