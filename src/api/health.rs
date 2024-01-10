use axum::{http::StatusCode, routing::get, Router};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/", get(health))
        .route("/health", get(health))
}

async fn health() -> StatusCode {
    StatusCode::OK
}
