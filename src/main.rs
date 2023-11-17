use axum::{Extension, Router, Server};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::net::SocketAddr;

mod api;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("\".env\" file is missing");
    let socket_address: SocketAddr = dotenvy::var("SOCKET_ADDRESS")
        .expect("\"SOCKET_ADDRESS\"  is missing from \".env\" file")
        .parse()
        .expect("\"SOCKET_ADDRESS\" is invalid, either IPv4 or IPv6");
    let database_url: String =
        dotenvy::var("DATABASE_URL").expect("\"DATABASE_URL\" is missing from \".env\" file");
    let aes_key: String =
        dotenvy::var("AES_KEY").expect("\"AES_KEY\" is missing from \".env\" file");

    let pool: Pool<MySql> = match MySqlPoolOptions::new()
        .max_connections(4)
        .connect(&database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => return eprintln!("{e}"),
    };

    let app = Router::new()
        .nest("/api", api::routes())
        .with_state(pool)
        .layer(Extension(aes_key));

    let server = Server::bind(&socket_address).serve(app.into_make_service());

    println!("\nsumire is alive @ http://{}/api\n", server.local_addr());

    server.await.expect("sumire died");
}
