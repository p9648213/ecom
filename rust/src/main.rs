use clap::Parser;
use rust::{config::Config, router::create_router};
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let config = Config::parse();

    let pool = SqlitePoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to database");

    let app = create_router(pool, config);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server is listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
