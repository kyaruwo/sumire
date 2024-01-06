use axum::{serve, Extension, Router};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

mod api;
mod app;
mod config;

#[tokio::main]
async fn main() {
    let config: config::Config = config::load().await;

    let db_pool: Pool<MySql> = match MySqlPoolOptions::new()
        .max_connections(4)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => return eprintln!("main > {e}"),
    };

    let app: Router = Router::new()
        .nest("/api", api::routes())
        .with_state(db_pool)
        .layer(Extension(config.aes_key))
        .nest("/app", app::routes());

    println!("\nsumire is alive\n");
    println!(" backend @ http://{}/api", config.address);
    println!("frontend @ http://{}/app", config.address);

    serve(config.tcp_listener, app.into_make_service())
        .await
        .expect("\nsumire died");
}
