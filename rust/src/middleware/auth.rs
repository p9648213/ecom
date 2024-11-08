use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use cookie::Cookie;

use crate::{
    config::Config,
    utilities::{app_error::AppError, jwt::validate_token, redirect::redirect_307},
};

#[derive(Clone)]
pub struct UserExtention {
    pub user: Option<UserInfo>,
}

#[derive(Clone)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub email: String,
}

pub async fn auth_user(
    State(config): State<Config>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    if request.uri().path() == "/" {
        return Ok(redirect_307("/shop/home"));
    }

    let cookie = request.headers().get("cookie");

    if let Some(cookie) = cookie {
        let token = cookie.to_str().map_err(|error| {
            tracing::error!("Failed to get token from cookie: {}", error);
            AppError::new(StatusCode::UNAUTHORIZED, "Invalid token".to_string())
        })?;

        let token: Vec<&str> = token.split("; ").collect();

        let token = token.iter().find(|token| token.starts_with("token="));

        if let Some(token) = token {
            let jwt_token = &token[6..token.chars().count()];

            let claims = validate_token(&config.jwt_secret, jwt_token);

            if let Some(claims) = claims {
                match claims.role.as_str() {
                    "admin" => match request.uri().path() {
                        "/auth/login" | "/auth/register" => Ok(redirect_307("/admin/dashboard")),
                        _ => {
                            if request.uri().path().contains("/shop") {
                                return Ok(redirect_307("/admin/dashboard"));
                            }

                            Ok(next.run(request).await.into_response())
                        }
                    },
                    "user" => match request.uri().path() {
                        "/auth/login" | "/auth/register" => Ok(redirect_307("/shop/home")),
                        _ => {
                            if request.uri().path().contains("/admin") {
                                return Err(AppError::new(
                                    StatusCode::UNAUTHORIZED,
                                    "You don't have permission to access this page".to_string(),
                                ));
                            }

                            request.extensions_mut().insert(UserExtention {
                                user: Some(UserInfo {
                                    id: claims.id,
                                    username: claims.username,
                                    email: claims.email,
                                }),
                            });

                            Ok(next.run(request).await.into_response())
                        }
                    },
                    _ => {
                        tracing::error!("Invalid user role: {}", claims.role);
                        Err(AppError::new(
                            StatusCode::UNAUTHORIZED,
                            "Server error".to_string(),
                        ))
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

                if request.uri().path().contains("admin") {
                    Ok((cookies, redirect_307("/auth/login")).into_response())
                } else {
                    request
                        .extensions_mut()
                        .insert(UserExtention { user: None });
                    Ok((cookies, next.run(request).await).into_response())
                }
            }
        } else {
            if request.uri().path().contains("admin") {
                Ok(redirect_307("/auth/login"))
            } else {
                request
                    .extensions_mut()
                    .insert(UserExtention { user: None });
                Ok(next.run(request).await.into_response())
            }
        }
    } else {
        if request.uri().path().contains("admin") {
            Ok(redirect_307("/auth/login"))
        } else {
            request
                .extensions_mut()
                .insert(UserExtention { user: None });
            Ok(next.run(request).await.into_response())
        }
    }
}
