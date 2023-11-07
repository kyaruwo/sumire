use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct WahIn {
    wah: bool,
}

#[derive(Serialize)]
pub struct WahOut {
    data: String,
}

pub async fn wah(Json(payload): Json<WahIn>) -> Json<WahOut> {
    let mut wah = WahOut {
        data: String::new(),
    };

    wah.data = match payload.wah {
        true => String::from("Ninomae Ina'nis is Cute"),
        false => String::from("not so wah?"),
    };

    Json(wah)
}
