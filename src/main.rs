mod api;
use axum::routing::{delete, get, post, put};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

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

    let app = axum::Router::new()
        .route("/api", get(api::health))
        .route("/api/wah", post(api::wah))
        .route("/api/notes", post(api::create_note))
        .route("/api/notes", get(api::read_notes))
        .route("/api/notes/:id", put(api::update_note))
        .route("/api/notes/:id", delete(api::delete_note))
        .with_state(pool);

    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 42069).into();
    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    println!("\nsumire is alive @ http://{}/api\n", server.local_addr());

    server.await.expect("sumire died");
}
