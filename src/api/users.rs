use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::{
    extract::{DefaultBodyLimit, State},
    http::StatusCode,
    response::Result,
    routing::post,
    Extension, Json, Router,
};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, FromRow, MySql, Pool};
use validator::Validate;
use {lazy_static::lazy_static, regex::Regex};

pub fn routes() -> Router<Pool<MySql>> {
    Router::new()
        .route("/users/register", post(register))
        .route("/users/login", post(login))
        .layer(DefaultBodyLimit::max(142))
}

lazy_static! {
    static ref USER_NAME: Regex = Regex::new(r"^[a-z]{4,20}$").expect("USER_NAME Regex Error");
}

#[derive(Deserialize, Validate)]
struct User {
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

async fn register(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
    Json(payload): Json<User>,
) -> Result<StatusCode> {
    match payload.validate() {
        Err(e) => return Err((StatusCode::BAD_REQUEST, Json(e)).into()),
        _ => (),
    };

    match sqlx::query("SELECT id FROM Users WHERE `name`=AES_ENCRYPT(?, ?);")
        .bind(&payload.name)
        .bind(&aes_key)
        .fetch_optional(&db_pool)
        .await
    {
        Ok(res) => match res {
            Some(_) => return Err(StatusCode::CONFLICT.into()),
            None => (),
        },
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    let password_hash: String = match Argon2::default().hash_password(
        payload.password.as_bytes(),
        &SaltString::generate(&mut OsRng),
    ) {
        Ok(password_hash) => password_hash.to_string(),
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match sqlx::query(
        "INSERT INTO Users (`name`, `password`) VALUES (AES_ENCRYPT(?, ?), AES_ENCRYPT(?, ?));",
    )
    .bind(&payload.name)
    .bind(&aes_key)
    .bind(&password_hash)
    .bind(&aes_key)
    .execute(&db_pool)
    .await
    {
        Ok(res) => {
            log(res.last_insert_id(), "register", &db_pool).await;
            Ok(StatusCode::CREATED)
        }
        Err(e) => {
            eprintln!("{e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR.into())
        }
    }
}

async fn login(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
    Json(payload): Json<User>,
) -> Result<(StatusCode, Json<Token>)> {
    match payload.validate() {
        Err(e) => return Err((StatusCode::BAD_REQUEST, Json(e)).into()),
        _ => (),
    };

    let user: Password = match sqlx::query_as::<_, Password>(
        "SELECT id, CONVERT(AES_DECRYPT(`password`, ?) USING utf8) as `password_hash` FROM Users WHERE `name`=AES_ENCRYPT(?, ?);",
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
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    let password_hash: PasswordHash<'_> = match PasswordHash::new(&user.password_hash) {
        Ok(password_hash) => password_hash,
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match Argon2::default().verify_password(payload.password.as_bytes(), &password_hash) {
        Err(_) => return Err(StatusCode::NOT_FOUND.into()),
        Ok(_) => (),
    }

    let token: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 420);

    let res: MySqlQueryResult = match sqlx::query(
        "UPDATE Users SET token=AES_ENCRYPT(?, ?) WHERE `name`=AES_ENCRYPT(?, ?);",
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
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match res.rows_affected() {
        1 => {
            log(user.id, "login", &db_pool).await;
            Ok((StatusCode::OK, Json(Token { token })))
        }
        _ => Err(StatusCode::NOT_FOUND.into()),
    }
}

async fn log(user_id: u64, action: &str, db_pool: &Pool<MySql>) {
    match sqlx::query("INSERT INTO Logs (`user_id`, `action`) VALUES (?, ?);")
        .bind(user_id)
        .bind(action)
        .execute(db_pool)
        .await
    {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    }
}
