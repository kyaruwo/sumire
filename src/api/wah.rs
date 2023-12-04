use axum::{extract::DefaultBodyLimit, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

pub fn routes() -> Router<Pool<MySql>> {
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
    data: &'static str,
}

async fn wah(Json(payload): Json<WahIn>) -> Json<WahOut> {
    let mut wah: WahOut = WahOut { data: "" };

    wah.data = match payload.wah {
        true => "Ninomae Ina'nis is Cute",
        false => "not so wah?",
    };

    Json(wah)
}
