use axum::{Router, Server};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::net::SocketAddr;

mod api;

#[tokio::main]
async fn main() {
    const DATABASE_URL: &str = "mysql://sumire:wah@127.0.0.1/sumire";

    let pool: Pool<MySql> = match MySqlPoolOptions::new()
        .max_connections(4)
        .connect(DATABASE_URL)
        .await
    {
        Ok(pool) => pool,
        Err(e) => return eprintln!("{e}"),
    };

    let app = Router::new().nest("/api", api::routes()).with_state(pool);

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 42069));
    let server = Server::bind(&addr).serve(app.into_make_service());

    println!("\nsumire is alive @ http://{}/api/\n", server.local_addr());

    server.await.expect("sumire died");
}
