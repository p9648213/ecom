use crate::config::Config;
use crate::controllers::auth::login_user::login_user;
use crate::controllers::auth::register_user::register_user;
use crate::middleware::auth::auth_user;
use crate::views::pages::admin::{admin_contents, admin_view};
use crate::views::pages::register::register_view;
use crate::views::pages::{home::home_view, login::login_view};
use axum::extract::FromRef;
use axum::http::header::CACHE_CONTROL;
use axum::http::HeaderValue;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Sqlite};
use tower_http::set_header::SetResponseHeaderLayer;

use std::path::PathBuf;
use tower_http::{compression::CompressionLayer, services::ServeDir};

#[derive(Clone, FromRef)]
pub struct AppState {
    pool: Pool<Sqlite>,
    config: Config,
}

pub fn create_router(pool: Pool<Sqlite>, config: Config) -> Router {
    let app_state = AppState { pool, config };
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let cache_control_layer = SetResponseHeaderLayer::if_not_present(
        CACHE_CONTROL,
        HeaderValue::from_static("no-cache, no-store, must-revalidate"),
    );

    Router::new()
        .route("/shop/home", get(home_view))
        .route("/admin/:admin_path", get(admin_view))
        .route("/admin/contents/:path", get(admin_contents))
        .route("/auth/register", get(register_view))
        .route("/auth/login", get(login_view))
        .route("/auth/register", post(register_user))
        .route("/auth/login", post(login_user))
        .route("/", get(ping))
        .with_state(app_state.clone())
        .layer(cache_control_layer)
        .layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            auth_user,
        ))
        .nest_service("/assets", ServeDir::new(assets_dir))
        .layer(CompressionLayer::new())
}

async fn ping() -> &'static str {
    "pong"
}
