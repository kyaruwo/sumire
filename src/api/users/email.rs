use axum::{routing::put, Router};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new().route("/email", put(update_email))
}

async fn update_email() {
    todo!()
}
