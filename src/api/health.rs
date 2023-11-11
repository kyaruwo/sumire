use axum::{body::Body, http::StatusCode, routing::get, Router};
use sqlx::{MySql, Pool};

pub fn routes() -> Router<Pool<MySql>, Body> {
    Router::new().route("/health", get(health))
}

async fn health() -> StatusCode {
    StatusCode::OK
}
