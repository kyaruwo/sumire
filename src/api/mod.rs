mod health;
mod notes;
mod users;
mod wah;

pub fn routes() -> axum::Router<sqlx::Pool<sqlx::MySql>> {
    axum::Router::new()
        .merge(health::routes())
        .merge(wah::routes())
        .merge(users::routes())
        .merge(notes::routes())
}
