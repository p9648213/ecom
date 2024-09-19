use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub exp: usize,
    pub email: String,
    pub role: String,
    pub id: i64,
}

pub fn create_token(
    secret: &str,
    email: &str,
    role: &str,
    id: i64,
    expires_minutes: i64,
) -> Result<String, AppError> {
    let now = chrono::Utc::now();
    let expires_duration = Duration::minutes(expires_minutes);
    let exp = (now + expires_duration).timestamp() as usize;

    let claims = Claims {
        exp,
        email: email.to_string(),
        role: role.to_string(),
        id
    };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&token_header, &claims, &key).map_err(|error| {
        tracing::error!("Failed to create token: {}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server error".to_string(),
        )
    })
}

pub fn validate_token(secret: &str, token: &str) -> Option<Claims> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

    let token = decode::<Claims>(token, &key, &validation);

    if let Ok(token) = token {
        return Some(token.claims);
    }

    None
}
