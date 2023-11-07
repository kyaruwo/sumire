use axum::{
    routing::{get, post},
    Json,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(root))
        .route("/wah", post(wah));

    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 42069).into();
    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    println!("server live: http://{}", server.local_addr());

    server.await.expect("server died");
}

async fn root() -> &'static str {
    "sumire"
}

async fn wah(Json(payload): Json<Wah>) -> Json<Wah> {
    let mut wah = Wah {
        data: String::new(),
    };

    if payload.data == "wah" {
        wah.data = String::from("Ninomae Ina'nis is Cute");
    } else {
        wah.data = String::from("wah doko?");
    }

    Json(wah)
}

#[derive(Serialize, Deserialize)]
struct Wah {
    data: String,
}
