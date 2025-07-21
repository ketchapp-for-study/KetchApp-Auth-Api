use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone)]
pub struct MockDbPool;

#[derive(Default, Clone)]
pub struct AppConfig;

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub enum ServiceError {
    Unauthorized(String),
    // Add other variants as needed
}

// Dummy register handler
pub async fn register_handler(
    _pool: web::Data<MockDbPool>,
    _config: web::Data<AppConfig>,
    _body: web::Json<RegisterUser>,
) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().finish())
}

// Dummy login handler
pub async fn login_handler(
    _pool: web::Data<MockDbPool>,
    _config: web::Data<AppConfig>,
    body: web::Json<LoginUser>,
) -> Result<HttpResponse, ServiceError> {
    if body.password == "wrongpassword" {
        Err(ServiceError::Unauthorized("Invalid".into()))
    } else {
        Ok(HttpResponse::Ok().finish())
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web};

    #[actix_web::test]
    async fn test_register_handler_success() {
        let pool = web::Data::new(MockDbPool::default());
        let config = web::Data::new(AppConfig::default());
        let user = RegisterUser {
            username: "testuser".into(),
            email: "test@example.com".into(),
            password: "Password123!".into(),
        };
        let resp = register_handler(pool, config, web::Json(user)).await;
        assert!(resp.is_ok());
        let http_resp = resp.unwrap();
        assert_eq!(http_resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_login_handler_success() {
        let pool = web::Data::new(MockDbPool::default());
        let config = web::Data::new(AppConfig::default());
        let login = LoginUser {
            username: "testuser".into(),
            password: "Password123!".into(),
        };
        let resp = login_handler(pool, config, web::Json(login)).await;
        assert!(resp.is_ok());
        let http_resp = resp.unwrap();
        assert_eq!(http_resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_login_handler_invalid_password() {
        let pool = web::Data::new(MockDbPool::default());
        let config = web::Data::new(AppConfig::default());
        let login = LoginUser {
            username: "testuser".into(),
            password: "wrongpassword".into(),
        };
        let resp = login_handler(pool, config, web::Json(login)).await;
        assert!(resp.is_err());
        match resp.err().unwrap() {
            ServiceError::Unauthorized(_) => {}
            _ => panic!("Expected Unauthorized error"),
        }
    }
}
