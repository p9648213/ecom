use axum::{
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    Form,
};
use axum_extra::extract::CookieJar;
use cookie::Cookie;
use sqlx::{query_as, Pool, Sqlite};

use crate::{
    config::Config,
    models::user::User,
    utilities::{app_error::AppError, hash::verify_password, jwt::create_token},
};

use super::LoginForm;

pub async fn login_user(
    State(pool): State<Pool<Sqlite>>,
    State(config): State<Config>,
    Form(login_form): Form<LoginForm>,
) -> impl IntoResponse {
    let user = query_as!(
        User,
        r#"
        SELECT * FROM users WHERE email = $1
        "#,
        login_form.email
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

    if let Some(user) = user {
        if !verify_password(&login_form.password, &user.password)? {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "Invalid password".to_string(),
            ));
        }

        let token = create_token(&config.jwt_secret, &user.email, &user.role, user.id, 60)?;

        let token_cookie: Cookie = Cookie::build(("token", token))
            .same_site(cookie::SameSite::Lax)
            .http_only(true)
            .path("/")
            .max_age(cookie::time::Duration::minutes(60))
            .into();

        let cookies = CookieJar::new().add(token_cookie);

        match user.role.as_str() {
            "user" => {
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header("HX-Location", "/shop/home")
                    .body(axum::body::Body::empty())
                    .unwrap();
                Ok((cookies, response))
            }
            "admin" => {
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header("HX-Location", "/admin/dashboard")
                    .body(axum::body::Body::empty())
                    .unwrap();
                Ok((cookies, response))
            }
            _ => {
                tracing::error!("Invalid role");
                Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error".to_string(),
                ))
            }
        }
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "User not found".to_string(),
        ));
    }
}
