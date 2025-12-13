use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}
#[derive(Debug)]
pub enum AuthError {
    TokenCreationFailed,
    InvalidToken,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::TokenCreationFailed => write!(f, "Failed to create token"),
            AuthError::InvalidToken => write!(f, "Invalid or expired token"),
        }
    }
}

impl std::error::Error for AuthError {}

#[derive(Debug, Clone)]
pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(jwt_secret: String) -> Self {
        Self { secret: jwt_secret }
    }

    pub fn generate_token(&self, user_id: &str) -> Result<String, AuthError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user_id.to_owned(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|_| AuthError::TokenCreationFailed)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
