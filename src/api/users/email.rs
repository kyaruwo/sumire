use axum::{
    routing::{post, put},
    Router,
};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/code", post(email_code))
        .route("/verify", put(verify_email))
        .route("/change", post(change_email))
        .route("/new", put(new_email))
}

async fn email_code() {
    todo!()
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
