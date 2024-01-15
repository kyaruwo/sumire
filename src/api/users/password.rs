use axum::{
    routing::{post, put},
    Router,
};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/password", put(update_password))
        .route("/forgot_password", post(forgot_password))
        .route("/new_password", put(new_password))
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
