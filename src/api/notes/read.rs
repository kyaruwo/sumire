use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/:id", get(read_note))
        .route("/", get(read_notes))
}

async fn read_note() {
    todo!()
}

async fn read_notes() {
    todo!()
}
