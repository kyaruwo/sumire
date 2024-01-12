use axum::{
    routing::{post, put},
    Router,
};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/verify_email", put(verify_email))
        .route("/change_email", post(change_email))
        .route("/new_email", put(new_email))
}

async fn verify_email() {
    todo!()
}

async fn change_email() {
    todo!()
}

async fn new_email() {
    todo!()
}
