use axum::{routing::post, Router};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new().route("/register", post(register))
}

async fn register() {
    todo!()
}
