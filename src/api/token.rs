use axum::{
    extract::State, http::StatusCode, response::Result, routing::put, Extension, Json, Router,
};
use axum_extra::extract::CookieJar;
use rand::distributions::{Alphanumeric, DistString};
use serde::Serialize;
use sqlx::{MySql, Pool};

pub fn routes() -> Router<Pool<MySql>> {
    Router::new().route("/token", put(token))
}

#[derive(Serialize)]
struct Token {
    token: String,
}

async fn token(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
    cookies: CookieJar,
) -> Result<(StatusCode, Json<Token>)> {
    let old_token: &str = match cookies.get("token") {
        Some(cookie) => cookie.value(),
        None => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    let new_token: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 420);

    match sqlx::query(
        "
        UPDATE
            Users
        SET
            token = AES_ENCRYPT(?, ?)
        WHERE
            token = AES_ENCRYPT(?, ?);
        ",
    )
    .bind(&new_token)
    .bind(&aes_key)
    .bind(&old_token)
    .bind(&aes_key)
    .execute(&db_pool)
    .await
    {
        Ok(res) => match res.rows_affected() {
            1 => {
                return Ok((StatusCode::OK, Json(Token { token: new_token })));
            }
            _ => return Err(StatusCode::UNAUTHORIZED.into()),
        },
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    }
}
