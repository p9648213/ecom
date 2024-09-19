use argon2::{
  password_hash::{
      PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng
  },
  Argon2,
  
};
use axum::http::StatusCode;

use super::app_error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
  let salt = SaltString::generate(&mut OsRng);
  let password_hash = Argon2::default()
      .hash_password(password.as_bytes(), &salt)
      .map_err(|e| {
        tracing::error!("Failed to hash password: {}", e);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error".to_string())
      })?;
  Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
  let password = password.as_bytes();
  let parsed_hash = PasswordHash::new(&password_hash)
      .map_err(|e| {
        tracing::error!("Failed to parsed password hash: {}", e);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error".to_string())
      })?;
  let password_match = Argon2::default()
      .verify_password(password, &parsed_hash)
      .is_ok();
  Ok(password_match)  
}

