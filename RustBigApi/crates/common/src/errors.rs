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
}

impl ServiceError {
    pub fn Unauthorized(message: String) -> Self {
        ServiceError::ValidationError(message)
    }
}

// Definiamo una struttura per la risposta JSON dell'errore
#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub(crate) code: u16, // Codice di stato HTTP
    pub(crate) error: String, // Descrizione dell'errore
    pub(crate) message: String, // Messaggio di errore dettagliato
}


// Implementiamo il trait ResponseError per il nostro errore custom.
// Questo permette ad Actix di convertire automaticamente ServiceError in una HttpResponse.
impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ServiceError::NotFound(_) => StatusCode::NOT_FOUND,
            ServiceError::Conflict(_) => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let (error, message) = match self {
            ServiceError::ValidationError(msg) => ("Validation Error", msg.clone()),
            ServiceError::NotFound(msg) => ("Not Found", msg.clone()),
            ServiceError::Conflict(msg) => ("Conflict", msg.clone()),
            ServiceError::InternalServerError => ("Internal Server Error", self.to_string()),
            ServiceError::BlockingError => ("Blocking Error", self.to_string()),
            ServiceError::DatabaseError(_) => ("Database Error", self.to_string()),
            ServiceError::PoolError(_) => ("Database Pool Error", self.to_string()),
        };
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            error: error.to_string(),
            message,
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

// Dobbiamo gestire la conversione da BlockingError manualmente
// perché thiserror non può derivare From per errori che non implementano std::error::Error.
impl From<actix_web::error::BlockingError> for ServiceError {
    fn from(_: actix_web::error::BlockingError) -> Self {
        ServiceError::BlockingError
    }
}