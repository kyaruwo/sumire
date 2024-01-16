use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/notes", post(write_note))
        .route("/notes", get(read_notes))
        .route("/notes/:id", get(read_note))
        .route("/notes/:id", put(update_note))
        .route("/notes/:id", delete(delete_note))
}

async fn write_note() {
    todo!()
}

async fn read_notes() {
    todo!()
}

async fn read_note() {
    todo!()
}

async fn update_note() {
    todo!()
}

async fn delete_note() {
    todo!()
}
