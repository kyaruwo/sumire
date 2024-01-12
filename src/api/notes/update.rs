use axum::{routing::put, Router};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new().route("/:id", put(update_note))
}

async fn update_note() {
    todo!()
}
