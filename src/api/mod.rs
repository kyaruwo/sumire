mod health;
mod notes;
mod users;

pub fn routes() -> axum::Router<sqlx::Pool<sqlx::Postgres>> {
    axum::Router::new()
        .merge(health::routes())
        .nest("/users", users::routes())
        .nest("/notes", notes::routes())
}
