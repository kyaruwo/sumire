use axum::{
    routing::{post, put},
    Router,
};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/", put(update_password))
        .route("/forgot", post(forgot_password))
        .route("/new", put(new_password))
}

async fn update_password() {
    todo!()
}

async fn forgot_password() {
    todo!()
}

async fn new_password() {
    todo!()
}
