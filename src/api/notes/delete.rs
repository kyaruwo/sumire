use axum::{routing::delete, Router};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new().route("/:id", delete(delete_note))
}

async fn delete_note() {
    todo!()
}
