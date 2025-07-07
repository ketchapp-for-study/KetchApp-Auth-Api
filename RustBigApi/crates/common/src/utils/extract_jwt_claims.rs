use actix_web::HttpRequest;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::models::claims::Claims;

/// Extracts and validates a JWT from the Authorization header or auth_token cookie in an HttpRequest.
/// Returns Ok(Some(Claims)) if valid, Ok(None) if not found, Err if invalid.
pub fn extract_jwt_claims_from_request(req: &HttpRequest, secret: &str) -> Result<Option<Claims>, jsonwebtoken::errors::Error> {
    // Try to extract the JWT token from the Authorization header (preferred) or from the auth_token cookie
    let auth_header = req.headers().get("Authorization").and_then(|h| h.to_str().ok()).map(|s| s.to_owned());
    let cookie_token = req.cookie("auth_token").map(|c| c.value().to_owned());
    // Prefer Bearer token from header, fallback to cookie
    let token_opt = auth_header
        .as_deref()
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|s| s.to_owned())
        .or(cookie_token);
    if let Some(token) = token_opt {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_aud = false;
        let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_bytes()), &validation)?;
        Ok(Some(token_data.claims))
    } else {
        Ok(None)
    }
}

