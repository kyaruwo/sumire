mod delete;
mod read;
mod update;
mod write;

pub fn routes() -> axum::Router<sqlx::Pool<sqlx::Postgres>> {
    axum::Router::new()
        .merge(delete::routes())
        .merge(read::routes())
        .merge(update::routes())
        .merge(write::routes())
}
