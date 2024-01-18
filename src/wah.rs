use axum::{routing::get, Json, Router};
use serde::Serialize;

pub fn routes() -> Router {
    Router::new().route("/", get(wah))
}

#[derive(Serialize)]
struct Wah {
    wah: &'static str,
}

async fn wah() -> Json<Wah> {
    Json(Wah {
        wah: "Ninomae Ina'nis is Cute",
    })
}
