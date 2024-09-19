use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use sqlx::{query, query_as, Pool, Sqlite};

use crate::{models::user::User, utilities::{app_error::AppError, hash::hash_password}};

use super::RegisterForm;

pub async fn register_user(
    State(pool): State<Pool<Sqlite>>,
    Form(register_form): Form<RegisterForm>,
) -> impl IntoResponse {
    if register_form.username.is_empty() || register_form.email.is_empty() || register_form.password.is_empty() {
        return Err(AppError::new(StatusCode::BAD_REQUEST, "Please fill all fields".to_string()));
    }

    let result_check_user = query_as!(
        User,
        r#"
        SELECT * FROM users WHERE email = $1
        "#,
        register_form.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to check user: {}", err);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error".to_string(),
        )
    })?;

    match result_check_user {
        Some(_) => Err(AppError::new(
            StatusCode::CONFLICT,
            "User already exists".to_string(),
        )),
        None => {
            let password_hash = hash_password(&register_form.password)?;

            query!(
                r#"
                INSERT INTO users (username, email, password)
                VALUES ($1, $2, $3)
                "#,
                register_form.username,
                register_form.email,
                password_hash,
            )
            .execute(&pool)
            .await
            .map_err(|err| {
                tracing::error!("Failed to register user: {}", err);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error".to_string(),
                )
            })?;

            Ok(
                [
                    ("HX-Trigger", r#"{"toastmessage":{"type":"success","message":"User create successfully"}}"#),
                ],
            )
        }
    }
}
