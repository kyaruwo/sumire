use axum::{http::StatusCode, routing::get, Router};
use sqlx::{MySql, Pool};

pub fn routes() -> Router<Pool<MySql>> {
    Router::new()
        .route("/", get(health))
        .route("/health", get(health))
}

async fn health() -> StatusCode {
    StatusCode::OK
}
