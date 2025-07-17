use jsonwebtoken::{Algorithm, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: usize,  // Expiration timestamp
    pub iat: usize,  // Issued at timestamp
    pub iss: String, // Issuer
    pub aud: String, // Audience
}

impl Claims {
    pub fn generate_jwt(
        &self,
        private_key: &[u8],
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let header = Header::new(Algorithm::RS256);
        let encoding_key = jsonwebtoken::EncodingKey::from_rsa_pem(private_key)?;
        jsonwebtoken::encode(&header, self, &encoding_key)
    }
}
