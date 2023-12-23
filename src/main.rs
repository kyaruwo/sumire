use axum::{serve, Extension, Router};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod api;
mod config;

#[tokio::main]
async fn main() {
    let config: config::Config = config::load();

    let address: TcpListener = TcpListener::bind(&config.address)
        .await
        .expect("\"ADDRESS\" is invalid, either IPv4 or IPv6");

    let db_pool: Pool<MySql> = match MySqlPoolOptions::new()
        .max_connections(4)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => return eprintln!("{e}"),
    };

    let app: Router = Router::new()
        .nest("/api", api::routes())
        .with_state(db_pool)
        .layer(Extension(config.aes_key))
        .nest_service("/app", ServeDir::new("app"));

    println!("\nsumire is alive\n");
    println!(" backend @ http://{}/api", config.address);
    println!("frontend @ http://{}/app", config.address);

    serve(address, app.into_make_service())
        .await
        .expect("sumire died");
}
