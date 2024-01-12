use axum::{routing::post, Router};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new().route("/login", post(login))
}

async fn login() {
    todo!()
}
