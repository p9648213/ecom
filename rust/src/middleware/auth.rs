use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use cookie::Cookie;
use sqlx::{query_as, Pool, Sqlite};

use crate::{
    config::Config,
    models::user::User,
    utilities::{app_error::AppError, jwt::validate_token},
};

pub async fn auth_user(
    State(config): State<Config>,
    State(pool): State<Pool<Sqlite>>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let cookie = request.headers().get("cookie");

    if let Some(cookie) = cookie {
        let token = cookie.to_str().map_err(|error| {
            tracing::error!("Failed to get token from cookie: {}", error);
            AppError::new(StatusCode::UNAUTHORIZED, "Invalid token".to_string())
        })?;

        let token = &token[6..token.chars().count()];

        let claims = validate_token(&config.jwt_secret, token);

        if let Some(claims) = claims {
            let user = query_as!(
                User,
                r#"
                SELECT * FROM users WHERE id = $1
                "#,
                claims.id
            )
            .fetch_optional(&pool)
            .await
            .map_err(|error| {
                tracing::error!("Failed to get user from database: {}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server error".to_string(),
                )
            })?;

            if let Some(user) = user {
                match user.role.as_str() {
                    "admin" => match request.uri().path() {
                        "/auth/login" | "/auth/register" => {
                            Ok(redirect_307_no_cache("/admin/dashboard"))
                        }
                        _ => {
                            if request.uri().path().contains("/shop") {
                                return Ok(redirect_307_no_cache("/admin/dashboard"));
                            }

                            Ok(next.run(request).await.into_response())
                        }
                    },
                    "user" => match request.uri().path() {
                        "/auth/login" | "/auth/register" => Ok(redirect_307_no_cache("/shop/home")),
                        _ => {
                            if request.uri().path().contains("/admin") {
                                return Err(AppError::new(
                                    StatusCode::UNAUTHORIZED,
                                    "You don't have permission to access this page".to_string(),
                                ));
                            }

                            Ok(next.run(request).await.into_response())
                        }
                    },
                    _ => {
                        tracing::error!("Invalid user role: {}", user.role);
                        Err(AppError::new(
                            StatusCode::UNAUTHORIZED,
                            "Server error".to_string(),
                        ))
                    }
                }
            } else {
                match request.uri().path() {
                    "/auth/login" | "/auth/register" => Ok(next.run(request).await.into_response()),
                    _ => Ok(redirect_307_no_cache("/auth/login")),
                }
            }
        } else {
            let token_cookie: Cookie = Cookie::build(("token", ""))
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .path("/")
                .max_age(cookie::time::Duration::minutes(0))
                .into();
            let cookies = CookieJar::new().add(token_cookie);
            Ok((cookies, redirect_307_no_cache("/auth/login")).into_response())
        }
    } else {
        match request.uri().path() {
            "/auth/login" | "/auth/register" => Ok(next.run(request).await.into_response()),
            _ => Ok(redirect_307_no_cache("/auth/login")),
        }
    }
}

fn redirect_307_no_cache(location: &str) -> Response {
    Response::builder()
        .status(307)
        .header(header::LOCATION, location)
        .header(header::CACHE_CONTROL, "no-cache")
        .body(axum::body::Body::empty())
        .unwrap()
}
