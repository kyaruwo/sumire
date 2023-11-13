use axum::{body::Body, extract::DefaultBodyLimit, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

pub fn routes() -> Router<Pool<MySql>, Body> {
    Router::new()
        .route("/wah", post(wah))
        .layer(DefaultBodyLimit::max(22))
}

#[derive(Deserialize)]
struct WahIn {
    wah: bool,
}

#[derive(Serialize)]
struct WahOut {
    data: String,
}

async fn wah(Json(payload): Json<WahIn>) -> Json<WahOut> {
    let mut wah: WahOut = WahOut {
        data: String::new(),
    };

    wah.data = match payload.wah {
        true => String::from("Ninomae Ina'nis is Cute"),
        false => String::from("not so wah?"),
    };

    Json(wah)
}
