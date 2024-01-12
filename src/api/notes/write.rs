use axum::{routing::post, Router};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new().route("/", post(write_note))
}

async fn write_note() {
    todo!()
}
