use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: usize,  // Expiration timestamp
    pub iat: usize,  // Issued at timestamp
    pub iss: String, // Issuer
    pub aud: String, // Audience
    pub roles: Vec<String>, // User roles
}

impl Claims {
    pub fn generate_jwt(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let header = jsonwebtoken::Header::default();
        let encoding_key = jsonwebtoken::EncodingKey::from_secret(secret.as_ref());
        jsonwebtoken::encode(&header, self, &encoding_key)
    }
}
