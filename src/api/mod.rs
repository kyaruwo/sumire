mod health;
mod log;
mod notes;
mod token;
mod users;
mod wah;

pub fn routes() -> axum::Router<sqlx::Pool<sqlx::MySql>> {
    axum::Router::new()
        .merge(health::routes())
        .merge(wah::routes())
        .merge(users::routes())
        .merge(notes::routes())
        .merge(token::routes())
}
