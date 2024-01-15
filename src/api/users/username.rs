use axum::{routing::put, Router};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new().route("/", put(update_username))
}

async fn update_username() {
    todo!()
}
