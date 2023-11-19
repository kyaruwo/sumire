use axum::{Extension, Router, Server};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

mod api;
mod config;

#[tokio::main]
async fn main() {
    let config: config::Config = config::load();

    let pool: Pool<MySql> = match MySqlPoolOptions::new()
        .max_connections(4)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => return eprintln!("{e}"),
    };

    let app = Router::new()
        .nest("/api", api::routes())
        .with_state(pool)
        .layer(Extension(config.aes_key));

    let server = Server::bind(&config.socket_address).serve(app.into_make_service());

    println!("\nsumire is alive @ http://{}/api\n", server.local_addr());

    server.await.expect("sumire died");
}
