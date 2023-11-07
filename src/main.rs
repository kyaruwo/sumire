mod api;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
};

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/api", get(api::health))
        .route("/api/wah", post(api::wah))
        .layer(DefaultBodyLimit::max(420));

    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 42069).into();
    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    println!("sumire is alive @ http://{}", server.local_addr());

    server.await.expect("sumire died");
}
