mod email;
mod login;
mod password;
mod register;
mod username;

pub fn routes() -> axum::Router<sqlx::Pool<sqlx::Postgres>> {
    axum::Router::new()
        .nest("/register", register::routes())
        .nest("/login", login::routes())
        .nest("/email", email::routes())
        .nest("/username", username::routes())
        .nest("/password", password::routes())
}
