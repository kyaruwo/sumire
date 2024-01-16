use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::Result,
    routing::{post, put},
    Json, Router,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use validator::Validate;

use {lazy_static::lazy_static, regex::Regex};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/register", post(register))
        .route("/email_code", post(email_code))
        .route("/verify_email", put(verify_email))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/change_email", post(change_email))
        .route("/new_email", put(new_email))
        .route("/username", put(update_username))
        .route("/password", put(update_password))
        .route("/forgot_password", post(forgot_password))
        .route("/new_password", put(new_password))
}

lazy_static! {
    static ref EMAIL: Regex = Regex::new(r"^[a-z0-9](\.?[a-z0-9]){5,29}\@(gmail|googlemail)\.com$")
        .expect("EMAIL Regex Error");
    static ref USERNAME: Regex = Regex::new(r"^[a-z]{4,20}$").expect("USERNAME Regex Error");
}

#[derive(Deserialize, Validate)]
struct Register {
    #[validate(
        regex(path = "EMAIL", code = "invalid", message = "only_google"),
        length(min = 16, max = 45, message = "length_email")
    )]
    email: String,
    #[validate(
        regex(path = "USERNAME", code = "invalid", message = "invalid_username"),
        length(min = 4, max = 20, message = "length_name")
    )]
    username: String,
    #[validate(length(min = 11, max = 69, message = "length_password"))]
    password: String,
}

async fn register(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<Register>,
) -> Result<StatusCode> {
    match payload.validate() {
        Err(e) => return Err(Json(e).into()),
        Ok(_) => (),
    }

    let password_hash: String = match Argon2::default().hash_password(
        payload.password.as_bytes(),
        &SaltString::generate(&mut OsRng),
    ) {
        Ok(password_hash) => password_hash.to_string(),
        Err(e) => {
            eprintln!("users > register > password_hash > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    let error: sqlx::Error = match sqlx::query(
        "
    INSERT INTO
        USERS (
            USER_ID,
            EMAIL,
            USERNAME,
            PASSWORD_HASH,
            CREATED_AT
        )
    VALUES
        ($1, $2, $3, $4, $5);
    ",
    )
    .bind(Uuid::new_v4())
    .bind(payload.email)
    .bind(payload.username)
    .bind(password_hash)
    .bind(Utc::now())
    .execute(&pool)
    .await
    {
        Ok(_) => return Ok(StatusCode::CREATED),
        Err(e) => e,
    };

    match error.as_database_error() {
        Some(e) => {
            if e.constraint() == Some("users_username_key") {
                return Err((StatusCode::CONFLICT, "USERNAME").into());
            }
            if e.constraint() == Some("users_email_key") {
                return Err((StatusCode::CONFLICT, "EMAIL").into());
            }
            if e.constraint() == Some("users_pkey") {
                return Err((StatusCode::CONFLICT, "UUID").into());
            }
        }
        None => (),
    }

    eprintln!("users > register > error > {error}");
    Err(StatusCode::INTERNAL_SERVER_ERROR.into())
}

async fn email_code() {
    todo!()
}

async fn verify_email() {
    todo!()
}

async fn login() {
    todo!()
}

async fn logout() {
    todo!()
}

async fn change_email() {
    todo!()
}

async fn new_email() {
    todo!()
}

async fn update_username() {
    todo!()
}

async fn update_password() {
    todo!()
}

async fn forgot_password() {
    todo!()
}

async fn new_password() {
    todo!()
}
