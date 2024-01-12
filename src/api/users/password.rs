use axum::{
    routing::{post, put},
    Router,
};
use sqlx::{Pool, Postgres};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/forgot_password", post(forgot_password))
        .route("/new_password", put(new_password))
        .route("/change_password", put(change_password))
}

async fn forgot_password() {
    todo!()
}

async fn new_password() {
    todo!()
}

async fn change_password() {
    todo!()
}
