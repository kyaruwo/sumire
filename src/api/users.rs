use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::{
    extract::{DefaultBodyLimit, State},
    http::StatusCode,
    response::Result,
    routing::{post, put},
    Extension, Json, Router,
};
use axum_extra::extract::CookieJar;
use rand::{
    distributions::{Alphanumeric, DistString},
    Rng,
};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, FromRow, MySql, Pool};
use validator::Validate;
use {lazy_static::lazy_static, regex::Regex};

use crate::config::SMTP;

pub fn routes() -> Router<Pool<MySql>> {
    Router::new()
        .route("/users/register", post(register))
        .route("/users/request_code", post(request_code))
        .route("/users/verify_email", post(verify_email))
        .route("/users/forgot_password", post(forgot_password))
        .route("/users/new_password", put(new_password))
        .route("/users/login", post(login))
        .route("/users/token", put(token))
        .route("/users/change_email", post(change_email))
        .route("/users/new_email", put(new_email))
        .route("/users/name", put(update_name))
        .route("/users/password", put(update_password))
        .layer(DefaultBodyLimit::max(222))
}

lazy_static! {
    static ref EMAIL: Regex = Regex::new(r"^[a-z0-9](\.?[a-z0-9]){5,29}\@(gmail|googlemail)\.com$")
        .expect("EMAIL Regex Error");
    static ref USER_NAME: Regex = Regex::new(r"^[a-z]{4,20}$").expect("USER_NAME Regex Error");
}

#[derive(Deserialize, Validate)]
struct RegisterUser {
    #[validate(
        regex(path = "EMAIL", code = "invalid", message = "only_google"),
        length(min = 16, max = 45, message = "length_email")
    )]
    email: String,
    #[validate(
        regex(path = "USER_NAME", code = "invalid", message = "invalid_name"),
        length(min = 4, max = 20, message = "length_name")
    )]
    name: String,
    #[validate(length(min = 8, max = 69, message = "length_password"))]
    password: String,
}

#[derive(Serialize)]
struct Conflict {
    cause: &'static str,
}

async fn register(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
    Json(payload): Json<RegisterUser>,
) -> Result<StatusCode> {
    match payload.validate() {
        Err(e) => return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(e)).into()),
        _ => (),
    };

    match sqlx::query(
        "
        SELECT
            id
        FROM
            Users
        WHERE
            `email` = AES_ENCRYPT(?, ?);
        ",
    )
    .bind(&payload.email)
    .bind(&aes_key)
    .fetch_optional(&db_pool)
    .await
    {
        Ok(res) => match res {
            Some(_) => return Err((StatusCode::CONFLICT, Json(Conflict { cause: "email" })).into()),
            None => (),
        },
        Err(e) => {
            eprintln!("users > register > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match sqlx::query(
        "
        SELECT
            id
        FROM
            Users
        WHERE
            `name` = AES_ENCRYPT(?, ?);
        ",
    )
    .bind(&payload.name)
    .bind(&aes_key)
    .fetch_optional(&db_pool)
    .await
    {
        Ok(res) => match res {
            Some(_) => return Err((StatusCode::CONFLICT, Json(Conflict { cause: "name" })).into()),
            None => (),
        },
        Err(e) => {
            eprintln!("users > register > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    let password_hash: String = match Argon2::default().hash_password(
        payload.password.as_bytes(),
        &SaltString::generate(&mut OsRng),
    ) {
        Ok(password_hash) => password_hash.to_string(),
        Err(e) => {
            eprintln!("users > register > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match sqlx::query(
        "
        INSERT INTO
            Users (`email`, `name`, `password`)
        VALUES
            (
                AES_ENCRYPT(?, ?),
                AES_ENCRYPT(?, ?),
                AES_ENCRYPT(?, ?)
            );
        ",
    )
    .bind(&payload.email)
    .bind(&aes_key)
    .bind(&payload.name)
    .bind(&aes_key)
    .bind(&password_hash)
    .bind(&aes_key)
    .execute(&db_pool)
    .await
    {
        Ok(res) => {
            log(res.last_insert_id(), "register", "created", &db_pool).await;
            Ok(StatusCode::CREATED)
        }
        Err(e) => {
            eprintln!("users > register > {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR.into())
        }
    }
}

#[derive(Deserialize, Validate)]
struct RequestCode {
    #[validate(
        regex(path = "EMAIL", code = "invalid", message = "only_google"),
        length(min = 16, max = 45, message = "length_email")
    )]
    email: String,
}

#[derive(FromRow)]
struct CodeLimit {
    id: u64,
    code_limit: u64,
}

async fn request_code(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
    Extension(smtp): Extension<SMTP>,
    Json(payload): Json<RequestCode>,
) -> Result<StatusCode> {
    match payload.validate() {
        Err(e) => return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(e)).into()),
        _ => (),
    };

    let user: CodeLimit = match sqlx::query_as::<_, CodeLimit>(
        "
        SELECT
            id,
            `code_limit`
        FROM
            Users
        WHERE
            `email` = AES_ENCRYPT(?, ?);
        ",
    )
    .bind(&payload.email)
    .bind(&aes_key)
    .fetch_optional(&db_pool)
    .await
    {
        Ok(user) => match user {
            Some(user) => user,
            None => return Err(StatusCode::NOT_FOUND.into()),
        },
        Err(e) => {
            eprintln!("users > send_code > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    if user.code_limit >= 4 {
        return Err(StatusCode::TOO_MANY_REQUESTS.into());
    }

    let code: u64 = rand::thread_rng().gen_range(10000000..99999999);

    match sqlx::query(
        "
        UPDATE
            Users
        SET
            `code` = AES_ENCRYPT(?, ?),
            `code_limit` = `code_limit` + 1
        WHERE
            `email` = AES_ENCRYPT(?, ?);
        ",
    )
    .bind(&code)
    .bind(&aes_key)
    .bind(&payload.email)
    .bind(&aes_key)
    .execute(&db_pool)
    .await
    {
        Ok(res) => match res.rows_affected() {
            1 => (),
            _ => return Err(StatusCode::NOT_FOUND.into()),
        },
        Err(e) => {
            eprintln!("users > send_code > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match smtp.send_code(payload.email, code) {
        Ok(()) => {
            log(user.id, "send_code", "success", &db_pool).await;
            Ok(StatusCode::OK)
        }
        Err(e) => return Err(e.into()),
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
    code: u64,
}

#[derive(FromRow)]
struct CodeRetry {
    id: u64,
    code: u64,
    code_retry: u64,
}

async fn verify_email(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
    Json(payload): Json<VerifyEmail>,
) -> Result<StatusCode> {
    match payload.validate() {
        Err(e) => return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(e)).into()),
        _ => (),
    };

    let user: CodeRetry = match sqlx::query_as::<_, CodeRetry>(
        "
        SELECT
            id,
            CONVERT(AES_DECRYPT(`code`, ?) USING utf8) as `code`,
            `code_retry`
        FROM
            Users
        WHERE
            `email` = AES_ENCRYPT(?, ?);
        ",
    )
    .bind(&payload.email)
    .bind(&aes_key)
    .fetch_optional(&db_pool)
    .await
    {
        Ok(user) => match user {
            Some(user) => user,
            None => return Err(StatusCode::NOT_FOUND.into()),
        },
        Err(e) => {
            eprintln!("users > verify_email > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    if user.code_retry >= 8 {
        return Err(StatusCode::TOO_MANY_REQUESTS.into());
    }

    match sqlx::query(
        "
        UPDATE
            Users
        SET
            `code_retry` = `code_retry` + 1
        WHERE
            `email` = AES_ENCRYPT(?, ?);
        ",
    )
    .bind(&payload.email)
    .bind(&aes_key)
    .execute(&db_pool)
    .await
    {
        Ok(res) => match res.rows_affected() {
            1 => (),
            _ => return Err(StatusCode::NOT_FOUND.into()),
        },
        Err(e) => {
            eprintln!("users > verify_email > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    if payload.code != user.code {
        return Err(StatusCode::NOT_FOUND.into());
    }

    match sqlx::query(
        "
        UPDATE
            Users
        SET
            `code` = NULL,
            `verified` = 1
        WHERE
            `email` = AES_ENCRYPT(?, ?);
        ",
    )
    .bind(&payload.email)
    .bind(&aes_key)
    .execute(&db_pool)
    .await
    {
        Ok(res) => match res.rows_affected() {
            1 => (),
            _ => return Err(StatusCode::NOT_FOUND.into()),
        },
        Err(e) => {
            eprintln!("users > verify_email > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    log(user.id, "verify_email", "success", &db_pool).await;
    Ok(StatusCode::OK)
}

#[derive(Deserialize, Validate)]
struct LoginUser {
    #[validate(
        regex(path = "USER_NAME", code = "invalid", message = "invalid_name"),
        length(min = 4, max = 20, message = "length_name")
    )]
    name: String,
    #[validate(length(min = 8, max = 69, message = "length_password"))]
    password: String,
}

#[derive(FromRow)]
struct Password {
    id: u64,
    password_hash: String,
}

#[derive(Serialize)]
struct Token {
    token: String,
}

async fn login(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
    Json(payload): Json<LoginUser>,
) -> Result<Json<Token>> {
    match payload.validate() {
        Err(e) => return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(e)).into()),
        _ => (),
    };

    let user: Password = match sqlx::query_as::<_, Password>(
        "
        SELECT
            id,
            CONVERT(AES_DECRYPT(`password`, ?) USING utf8) as `password_hash`
        FROM
            Users
        WHERE
            `name` = AES_ENCRYPT(?, ?);
        ",
    )
    .bind(&aes_key)
    .bind(&payload.name)
    .bind(&aes_key)
    .fetch_optional(&db_pool)
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
            log(user.id, "login", "failed", &db_pool).await;
            return Err(StatusCode::NOT_FOUND.into());
        }
        Ok(_) => (),
    }

    let token: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 420);

    let res: MySqlQueryResult = match sqlx::query(
        "
        UPDATE
            Users
        SET
            `token` = AES_ENCRYPT(?, ?)
        WHERE
            `name` = AES_ENCRYPT(?, ?);
        ",
    )
    .bind(&token)
    .bind(&aes_key)
    .bind(&payload.name)
    .bind(&aes_key)
    .execute(&db_pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("users > login > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match res.rows_affected() {
        1 => {
            log(user.id, "login", "success", &db_pool).await;
            Ok(Json(Token { token }))
        }
        _ => Err(StatusCode::NOT_FOUND.into()),
    }
}

async fn token(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
    cookies: CookieJar,
) -> Result<Json<Token>> {
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
            `token` = AES_ENCRYPT(?, ?)
        WHERE
            `token` = AES_ENCRYPT(?, ?);
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
                return Ok(Json(Token { token: new_token }));
            }
            _ => return Err(StatusCode::UNAUTHORIZED.into()),
        },
        Err(e) => {
            eprintln!("users > token > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    }
}

async fn change_email() {
    todo!()
}

async fn new_email() {
    todo!()
}

async fn update_name() {
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

async fn log(user_id: u64, request: &str, response: &str, db_pool: &Pool<MySql>) {
    match sqlx::query(
        "
        INSERT INTO
            UsersLogs (`user_id`, `request`, `response`)
        VALUES
            (?, ?, ?);
        ",
    )
    .bind(user_id)
    .bind(request)
    .bind(response)
    .execute(db_pool)
    .await
    {
        Ok(_) => (),
        Err(e) => eprintln!("users > log > {e}"),
    }
}
