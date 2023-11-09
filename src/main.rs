mod api;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

#[tokio::main]
async fn main() {
    const DB_URL: &str = "sumire.sqlite";
    if !Sqlite::database_exists(DB_URL).await.unwrap() {
        Sqlite::create_database(DB_URL).await.unwrap();

        let db: Pool<Sqlite> = SqlitePool::connect(DB_URL).await.unwrap();

        sqlx::query("CREATE TABLE Notes (title VARCHAR(69), body text);")
            .execute(&db)
            .await
            .unwrap();
    }
    let db: Pool<Sqlite> = match SqlitePool::connect(DB_URL).await {
        Ok(db) => db,
        Err(e) => return println!("{e}"),
    };

    let app = axum::Router::new()
        .route("/api", get(api::health))
        .route("/api/wah", post(api::wah))
        .layer(DefaultBodyLimit::max(420))
        .with_state(db);

    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 42069).into();
    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    println!("sumire is alive @ http://{}/api", server.local_addr());

    server.await.expect("sumire died");
}
