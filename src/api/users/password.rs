use axum::{
    routing::{post, put},
    Router,
};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/forgot_password", post(forgot_password))
        .route("/new_password", put(new_password))
        .route("/update_password", put(update_password))
}

async fn forgot_password() {
    todo!()
}

async fn new_password() {
    todo!()
}

async fn update_password() {
    todo!()
}
