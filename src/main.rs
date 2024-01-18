use axum::{serve, Extension, Router};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod api;
mod config;
mod smtp;
mod wah;

#[tokio::main]
async fn main() {
    let config = config::load().await;

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(42)
        .connect(&config.database_url)
        .await
        .expect("database connections failed");

    let smtp = smtp::load();

    let router: Router = Router::new()
        .nest("/api", api::routes())
        .with_state(pool)
        .layer(Extension(smtp))
        .nest("/wah", wah::routes());

    println!("\nsumire is alive\n");
    println!(" backend @ http://{}/api", config.address);

    serve(config.tcp_listener, router.into_make_service())
        .await
        .expect("\nsumire died\n");
}
