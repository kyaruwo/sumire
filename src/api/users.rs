use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::{
    body::Body, extract::State, http::StatusCode, response::Result, routing::post, Extension, Json,
    Router,
};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, Pool};
use validator::Validate;

pub fn routes() -> Router<Pool<MySql>, Body> {
    Router::new()
        .route("/users/register", post(register))
        .route("/users/login", post(login))
}

#[derive(Serialize, Deserialize, FromRow, Validate)]
struct User {
    #[validate(length(min = 4, message = "min_string"))]
    name: String,
    #[validate(length(min = 8, message = "min_string"))]
    password: String,
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

    let user: User = User {
        name: payload.name,
        password: password_hash,
    };

    match sqlx::query(
        "INSERT INTO Users (`name`, `password`) VALUES (AES_ENCRYPT(?, ?), AES_ENCRYPT(?, ?));",
    )
    .bind(&user.name)
    .bind(&aes_key)
    .bind(&user.password)
    .bind(&aes_key)
    .execute(&db_pool)
    .await
    {
        Ok(_) => return Ok(StatusCode::CREATED),
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };
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

    // verify name
    let password_hash: String = match sqlx::query_as::<_, User>(
        "SELECT CONVERT(AES_DECRYPT(`name`, ?) USING utf8) as `name`, CONVERT(AES_DECRYPT(`password`, ?) USING utf8) as `password` FROM Users WHERE `name`=AES_ENCRYPT(?, ?);",
    )
    .bind(&aes_key)
    .bind(&aes_key)
    .bind(payload.name)
    .bind(&aes_key)
    .fetch_optional(&db_pool)
    .await
    {
        Ok(res) => match res {
            Some(user) => user.password,
            None => return Err(StatusCode::NOT_FOUND.into()),
        },
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    // verify password
    let password_hash: PasswordHash<'_> = match PasswordHash::new(&password_hash) {
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

    Ok((StatusCode::OK, Json(Token { token })))
}
