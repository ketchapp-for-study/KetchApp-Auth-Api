use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use validator::{Validate, ValidationError};
use uuid::Uuid;

use utoipa::openapi::schema;
use common::models::user::NewUser;

#[derive(Validate, Serialize, Deserialize, ToSchema)]
#[schema(
    title = "Login User",
    description = "Login a user in the system",
    example = json!({"username": "johndoe", "email": "john_doe@gmail.com", "password": "Secret123!"})
)]
pub struct LoginUser {
    #[validate(custom(function = "validate_username"))]
    #[schema(min_length = 6, max_length = 16, pattern = "^[a-zA-Z]{6,16}$")]
    pub username: String,

    #[validate(custom(function = "validate_password"))]
    #[schema(
        min_length = 8,
        max_length = 32,
        pattern = "^[A-Za-z\\d@$!%*?&]{8,32}$"
    )]
    pub password: String,
}

pub fn validate_username_logic(username: &str) -> Result<(), String> {
    if username.len() < 6 || username.len() > 16 {
        return Err("Username must be 6-16 letters (a-z, A-Z)".into());
    }
    if !username.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err("Username must be 6-16 letters (a-z, A-Z)".into());
    }
    Ok(())
}

pub fn validate_password_logic(password: &str) -> Result<(), String> {
    if password.len() < 8 || password.len() > 32 {
        return Err("Password must be between 8 and 32 characters".into());
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err("Password must contain at least one lowercase letter".into());
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err("Password must contain at least one uppercase letter".into());
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err("Password must contain at least one digit".into());
    }
    if !password.chars().any(|c| "@$!%*?&".contains(c)) {
        return Err("Password must contain at least one special character (@$!%*?&)".into());
    }
    Ok(())
}

pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    validate_username_logic(username).map_err(|e| {
        let mut err = ValidationError::new("invalid_username");
        err.message = Some(e.to_string().into());
        err
    })
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if let Err(e) = validate_password_logic(password) {
        let mut err = ValidationError::new("invalid_password");
        err.message = Some(e.to_string().into());
        return Err(err);
    }
    Ok(())
}
