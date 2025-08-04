use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

use crate::models::user::NewUser;

#[derive(Validate, Serialize, Deserialize, ToSchema)]
#[schema(
    title = "Register User",
    description = "Register a user in the system",
    example = json!({"username": "johndoe", "email": "john_doe@gmail.com", "password": "Secret123!"})
)]
#[derive(Debug)]
pub struct RegisterUser {
    #[validate(custom(function = "validate_username"))]
    #[schema(min_length = 6, max_length = 32, pattern = "^[a-zA-Z]{6,16}$")]
    pub username: String,

    #[validate(custom(function = "validate_email"))]
    #[schema(format = "email", pattern = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$")]
    #[validate(email)]
    pub email: String,

    #[validate(custom(function = "validate_password"))]
    #[schema(
        min_length = 8,
        max_length = 32,
        pattern = "^[A-Za-z\\d@$!%*?&]{8,32}$"
    )]
    pub password: String,
}

impl From<RegisterUser> for NewUser {
    fn from(reg: RegisterUser) -> Self {
        NewUser {
            username: reg.username,
            email: reg.email,
            password: reg.password,
        }
    }
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

pub fn validate_email_logic(email: &str) -> Result<(), String> {
    if !email.is_ascii() || !email.contains('@') || !email.contains('.') {
        return Err("Email must be a valid ASCII email address".into());
    }
    if email.len() < 5 || email.len() > 254 {
        return Err("Email must be between 5 and 254 characters".into());
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
    if let Err(e) = validate_username_logic(username) {
        let mut err = ValidationError::new("invalid_username");
        err.message = Some(e.to_string().into());
        return Err(err);
    }
    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    if let Err(e) = validate_email_logic(email) {
        let mut err = ValidationError::new("invalid_email");
        err.message = Some(e.to_string().into());
        return Err(err);
    }
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if let Err(e) = validate_password_logic(password) {
        let mut err = ValidationError::new("invalid_password");
        err.message = Some(e.to_string().into());
        return Err(err);
    }
    Ok(())
}
