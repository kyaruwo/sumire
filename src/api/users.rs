use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::Result,
    routing::{post, put},
    Extension, Json, Router,
};
use chrono::Utc;
use rand::Rng;
use serde::Deserialize;
use sqlx::{postgres::PgQueryResult, Pool, Postgres};
use uuid::Uuid;
use validator::Validate;

use crate::smtp::SMTP;

use {lazy_static::lazy_static, regex::Regex};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/users/register", post(register))
        .route("/users/code_request", post(code_request))
        .route("/users/verify_email", put(verify_email))
        .route("/users/login", post(login))
        .route("/users/logout", post(logout))
        .route("/users/change_email", post(change_email))
        .route("/users/new_email", put(new_email))
        .route("/users/username", put(update_username))
        .route("/users/password", put(update_password))
        .route("/users/forgot_password", post(forgot_password))
        .route("/users/new_password", put(new_password))
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

#[derive(Deserialize, Validate)]
struct CodeRequest {
    #[validate(
        regex(path = "EMAIL", code = "invalid", message = "only_google"),
        length(min = 16, max = 45, message = "length_email")
    )]
    email: String,
}

async fn code_request(
    State(pool): State<Pool<Postgres>>,
    Extension(smtp): Extension<SMTP>,
    Json(payload): Json<CodeRequest>,
) -> Result<StatusCode> {
    match payload.validate() {
        Err(e) => return Err(Json(e).into()),
        Ok(_) => (),
    }

    let code: i64 = rand::thread_rng().gen_range(10000000..99999999);

    let res: PgQueryResult = match sqlx::query(
        "
    UPDATE
        USERS
    SET
        CODE = $1
    WHERE
        EMAIL = $2;
    ",
    )
    .bind(code)
    .bind(&payload.email)
    .execute(&pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("users > code_request > error > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    if res.rows_affected() != 1 {
        return Err(StatusCode::NOT_FOUND.into());
    }

    match smtp.send_code(payload.email, code) {
        Ok(res) => Ok(res),
        Err(e) => Err(e.into()),
    }
}

#[derive(Deserialize, Validate)]
struct VerifyEmail {
    #[validate(
        regex(path = "EMAIL", code = "invalid", message = "only_google"),
        length(min = 16, max = 45, message = "length_email")
    )]
    email: String,
    #[validate(range(min = 10000000, max = 99999999, message = "range_code"))]
    code: i64,
}

async fn verify_email(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<VerifyEmail>,
) -> Result<StatusCode> {
    match payload.validate() {
        Err(e) => return Err(Json(e).into()),
        Ok(_) => (),
    }

    let res: PgQueryResult = match sqlx::query(
        "
    UPDATE
        USERS
    SET
        CODE = NULL,
        VERIFIED = TRUE
    WHERE
        EMAIL = $1
        AND CODE = $2
        AND VERIFIED = FALSE;
    ",
    )
    .bind(payload.email)
    .bind(payload.code)
    .execute(&pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("users > verify_email > error > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match res.rows_affected() {
        1 => Ok(StatusCode::OK),
        _ => Err(StatusCode::NOT_FOUND.into()),
    }
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
