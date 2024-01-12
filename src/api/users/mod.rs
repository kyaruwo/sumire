mod email;
mod login;
mod password;
mod register;
mod username;

pub fn routes() -> axum::Router<sqlx::Pool<sqlx::Postgres>> {
    axum::Router::new()
        .merge(email::routes())
        .merge(login::routes())
        .merge(password::routes())
        .merge(register::routes())
        .merge(username::routes())
}
