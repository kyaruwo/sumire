use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::Result,
    routing::{post, put},
    Extension, Json, Router,
};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use rand::{
    distributions::{Alphanumeric, DistString},
    Rng,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, FromRow, Pool, Postgres};
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
        .route("/users/logout", put(logout))
        .route("/users/change_email_request", post(change_email_request))
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

    eprintln!("users > register > {error}");
    Err(StatusCode::INTERNAL_SERVER_ERROR.into())
}

#[derive(Deserialize, Validate)]
struct Email {
    #[validate(
        regex(path = "EMAIL", code = "invalid", message = "only_google"),
        length(min = 16, max = 45, message = "length_email")
    )]
    email: String,
}

async fn code_request(
    State(pool): State<Pool<Postgres>>,
    Extension(smtp): Extension<SMTP>,
    Json(payload): Json<Email>,
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
        EMAIL = $2
        AND VERIFIED = FALSE;
    ",
    )
    .bind(code)
    .bind(&payload.email)
    .execute(&pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("users > code_request > {e}");
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
            eprintln!("users > verify_email > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match res.rows_affected() {
        1 => Ok(StatusCode::OK),
        _ => Err(StatusCode::NOT_FOUND.into()),
    }
}

#[derive(Deserialize, Validate)]
struct Login {
    #[validate(
        regex(path = "USERNAME", code = "invalid", message = "invalid_username"),
        length(min = 4, max = 20, message = "length_name")
    )]
    username: String,
    #[validate(length(min = 11, max = 69, message = "length_password"))]
    password: String,
}

#[derive(FromRow)]
struct Password {
    password_hash: String,
}

#[derive(Serialize)]
struct SessionID {
    session_id: String,
}

async fn login(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<Login>,
) -> Result<Json<SessionID>> {
    match payload.validate() {
        Err(e) => return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(e)).into()),
        _ => (),
    };

    let user: Password = match sqlx::query_as::<_, Password>(
        "
        SELECT
            PASSWORD_HASH
        FROM
            USERS
        WHERE
            USERNAME = $1;
        ",
    )
    .bind(&payload.username)
    .fetch_optional(&pool)
    .await
    {
        Ok(res) => match res {
            Some(user) => user,
            None => return Err(StatusCode::NOT_FOUND.into()),
        },
        Err(e) => {
            eprintln!("users > login > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    let password_hash: PasswordHash<'_> = match PasswordHash::new(&user.password_hash) {
        Ok(password_hash) => password_hash,
        Err(e) => {
            eprintln!("users > login > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match Argon2::default().verify_password(payload.password.as_bytes(), &password_hash) {
        Err(_) => {
            return Err(StatusCode::NOT_FOUND.into());
        }
        Ok(_) => (),
    }

    let session_id: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 420);

    let res: Option<bool> = match sqlx::query_scalar!(
        "
    UPDATE
        USERS
    SET
        SESSION_ID = $1
    WHERE
        USERNAME = $2 RETURNING VERIFIED
    ",
        session_id,
        payload.username
    )
    .fetch_one(&pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("users > login > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    if res == Some(true) {
        return Ok(Json(SessionID { session_id }));
    }
    Err((StatusCode::UNAUTHORIZED).into())
}

async fn logout(State(pool): State<Pool<Postgres>>, cookies: CookieJar) -> StatusCode {
    let session_id: &str = match cookies.get("session_id") {
        Some(cookie) => cookie.value(),
        None => return StatusCode::UNAUTHORIZED,
    };

    let res: PgQueryResult = match sqlx::query(
        "
    UPDATE
        USERS
    SET
        SESSION_ID = NULL
    WHERE
        SESSION_ID = $1;
    ",
    )
    .bind(session_id)
    .execute(&pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("users > logout > {e}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    match res.rows_affected() {
        1 => StatusCode::OK,
        _ => StatusCode::NOT_FOUND,
    }
}

async fn change_email_request(
    State(pool): State<Pool<Postgres>>,
    Extension(smtp): Extension<SMTP>,
    cookies: CookieJar,
    Json(payload): Json<Email>,
) -> Result<StatusCode> {
    let session_id: &str = match cookies.get("session_id") {
        Some(cookie) => cookie.value(),
        None => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    match payload.validate() {
        Err(e) => return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(e)).into()),
        _ => (),
    };

    let code: i64 = rand::thread_rng().gen_range(10000000..99999999);

    let res: PgQueryResult = match sqlx::query(
        "
    UPDATE
        USERS
    SET
        CODE = $1
    WHERE
        EMAIL = $2
        AND SESSION_ID = $3
        AND VERIFIED = TRUE;
    ",
    )
    .bind(code)
    .bind(&payload.email)
    .bind(session_id)
    .execute(&pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("users > change_email_request > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    if res.rows_affected() != 1 {
        return Err(StatusCode::UNAUTHORIZED.into());
    }

    match smtp.send_code(payload.email, code) {
        Ok(res) => Ok(res),
        Err(e) => Err(e.into()),
    }
}

#[derive(Deserialize, Validate)]
struct NewEmail {
    #[validate(
        regex(path = "EMAIL", code = "invalid", message = "only_google"),
        length(min = 16, max = 45, message = "length_email")
    )]
    old_email: String,
    #[validate(
        regex(path = "EMAIL", code = "invalid", message = "only_google"),
        length(min = 16, max = 45, message = "length_email")
    )]
    new_email: String,
    #[validate(range(min = 10000000, max = 99999999, message = "range_code"))]
    code: i64,
}

async fn new_email(
    State(pool): State<Pool<Postgres>>,
    cookies: CookieJar,
    Json(payload): Json<NewEmail>,
) -> Result<StatusCode> {
    let session_id: &str = match cookies.get("session_id") {
        Some(cookie) => cookie.value(),
        None => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    match payload.validate() {
        Err(e) => return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(e)).into()),
        _ => (),
    };

    let error: sqlx::Error = match sqlx::query(
        "
    UPDATE
        USERS
    SET
        CODE = NULL,
        EMAIL = $1
    WHERE
        EMAIL = $2
        AND CODE = $3
        AND SESSION_ID = $4
        AND VERIFIED = TRUE;
    ",
    )
    .bind(payload.new_email)
    .bind(payload.old_email)
    .bind(payload.code)
    .bind(session_id)
    .execute(&pool)
    .await
    {
        Ok(res) => match res.rows_affected() {
            1 => return Ok(StatusCode::OK),
            _ => return Err(StatusCode::NOT_FOUND.into()),
        },
        Err(e) => e,
    };

    match error.as_database_error() {
        Some(e) => {
            if e.constraint() == Some("users_email_key") {
                return Err((StatusCode::CONFLICT, "EMAIL").into());
            }
        }
        None => (),
    }

    eprintln!("users > new_email > {error}");
    Err(StatusCode::INTERNAL_SERVER_ERROR.into())
}

#[derive(Deserialize, Validate)]
struct UpdateUsername {
    #[validate(
        regex(path = "USERNAME", code = "invalid", message = "invalid_username"),
        length(min = 4, max = 20, message = "length_name")
    )]
    username: String,
}

async fn update_username(
    State(pool): State<Pool<Postgres>>,
    cookies: CookieJar,
    Json(payload): Json<UpdateUsername>,
) -> Result<StatusCode> {
    let session_id: &str = match cookies.get("session_id") {
        Some(cookie) => cookie.value(),
        None => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    match payload.validate() {
        Err(e) => return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(e)).into()),
        _ => (),
    };

    let error: sqlx::Error = match sqlx::query(
        "
    UPDATE
        USERS
    SET
        USERNAME = $1
    WHERE
        SESSION_ID = $2
        AND VERIFIED = TRUE;
    ",
    )
    .bind(payload.username)
    .bind(session_id)
    .execute(&pool)
    .await
    {
        Ok(res) => match res.rows_affected() {
            1 => return Ok(StatusCode::OK),
            _ => return Err(StatusCode::UNAUTHORIZED.into()),
        },
        Err(e) => e,
    };

    match error.as_database_error() {
        Some(e) => {
            if e.constraint() == Some("users_username_key") {
                return Err((StatusCode::CONFLICT, "USERNAME").into());
            }
        }
        None => (),
    }

    eprintln!("users > update_username > {error}");
    Err(StatusCode::INTERNAL_SERVER_ERROR.into())
}

#[derive(Deserialize, Validate)]
struct UpdatePassword {
    #[validate(length(min = 11, max = 69, message = "length_password"))]
    old_password: String,
    #[validate(length(min = 11, max = 69, message = "length_password"))]
    new_password: String,
}

async fn update_password(
    State(pool): State<Pool<Postgres>>,
    cookies: CookieJar,
    Json(payload): Json<UpdatePassword>,
) -> Result<StatusCode> {
    let session_id: &str = match cookies.get("session_id") {
        Some(cookie) => cookie.value(),
        None => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    match payload.validate() {
        Err(e) => return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(e)).into()),
        _ => (),
    };

    let user: Password = match sqlx::query_as::<_, Password>(
        "
        SELECT
            PASSWORD_HASH
        FROM
            USERS
        WHERE
            SESSION_ID = $1
            AND VERIFIED = TRUE;
        ",
    )
    .bind(session_id)
    .fetch_optional(&pool)
    .await
    {
        Ok(res) => match res {
            Some(user) => user,
            None => return Err(StatusCode::UNAUTHORIZED.into()),
        },
        Err(e) => {
            eprintln!("users > update_password > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    let password_hash: PasswordHash<'_> = match PasswordHash::new(&user.password_hash) {
        Ok(password_hash) => password_hash,
        Err(e) => {
            eprintln!("users > update_password > password_hash > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match Argon2::default().verify_password(payload.old_password.as_bytes(), &password_hash) {
        Err(_) => {
            return Err(StatusCode::NOT_FOUND.into());
        }
        Ok(_) => (),
    }

    let password_hash: String = match Argon2::default().hash_password(
        payload.new_password.as_bytes(),
        &SaltString::generate(&mut OsRng),
    ) {
        Ok(password_hash) => password_hash.to_string(),
        Err(e) => {
            eprintln!("users > update_password > password_hash > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match sqlx::query(
        "
    UPDATE
        USERS
    SET
        PASSWORD_HASH = $1
    WHERE
        SESSION_ID = $2
        AND VERIFIED = TRUE;
    ",
    )
    .bind(password_hash)
    .bind(session_id)
    .execute(&pool)
    .await
    {
        Ok(res) => match res.rows_affected() {
            1 => return Ok(StatusCode::OK),
            _ => return Err(StatusCode::UNAUTHORIZED.into()),
        },
        Err(e) => {
            eprintln!("users > update_password > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };
}

async fn forgot_password() {
    todo!()
}

async fn new_password() {
    todo!()
}
