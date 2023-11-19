use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Path, State},
    http::StatusCode,
    response::Result,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, FromRow, MySql, Pool};
use validator::Validate;

use crate::api::validation::empty_string;

pub fn routes() -> Router<Pool<MySql>, Body> {
    Router::new()
        .route("/notes", post(write_note))
        .route("/notes/:id", get(read_note))
        .route("/notes", get(read_notes))
        .route("/notes/:id", put(update_note))
        .route("/notes/:id", delete(delete_note))
        .layer(DefaultBodyLimit::max(690))
}

#[derive(Deserialize, Validate)]
struct WriteNote {
    #[validate(
        custom(function = "empty_string", message = "empty_string"),
        length(max = 42, message = "max_string")
    )]
    title: String,
    #[validate(
        custom(function = "empty_string", message = "empty_string"),
        length(max = 420, message = "max_string")
    )]
    body: String,
}

#[derive(Serialize, Deserialize, FromRow, Validate)]
struct Note {
    id: u64,
    #[validate(
        custom(function = "empty_string", message = "empty_string"),
        length(max = 42, message = "max_string")
    )]
    title: String,
    #[validate(
        custom(function = "empty_string", message = "empty_string"),
        length(max = 420, message = "max_string")
    )]
    body: String,
}

async fn write_note(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
    Json(payload): Json<WriteNote>,
) -> Result<(StatusCode, Json<Note>)> {
    match payload.validate() {
        Err(e) => return Err((StatusCode::BAD_REQUEST, Json(e)).into()),
        _ => (),
    };

    let mut note: Note = Note {
        id: 0,
        title: String::from(payload.title.trim()),
        body: String::from(payload.body.trim()),
    };

    note.id = match sqlx::query(
        "INSERT INTO Notes (title, body) VALUES (AES_ENCRYPT(?, ?), AES_ENCRYPT(?, ?));",
    )
    .bind(&note.title)
    .bind(&aes_key)
    .bind(&note.body)
    .bind(&aes_key)
    .execute(&db_pool)
    .await
    {
        Ok(res) => res.last_insert_id(),
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    Ok((StatusCode::CREATED, Json(note)))
}

async fn read_note(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
    Path(id): Path<u64>,
) -> Result<Json<Note>, StatusCode> {
    let res: Option<Note> = match sqlx::query_as::<_, Note>(
        "SELECT id, CONVERT(AES_DECRYPT(title, ?) USING utf8) as title, CONVERT(AES_DECRYPT(body, ?) USING utf8) as body FROM Notes WHERE id=?;",
    )
    .bind(&aes_key)
    .bind(&aes_key)
    .bind(id)
    .fetch_optional(&db_pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match res {
        Some(note) => Ok(Json(note)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn read_notes(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
) -> Result<Json<Vec<Note>>, StatusCode> {
    let notes: Vec<Note> = match sqlx::query_as::<_, Note>(
        "SELECT id, CONVERT(AES_DECRYPT(title, ?) USING utf8) as title, CONVERT(AES_DECRYPT(body, ?) USING utf8) as body FROM Notes;",
    )
    .bind(&aes_key)
    .bind(&aes_key)
    .fetch_all(&db_pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(notes))
}

async fn update_note(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
    Path(id): Path<u64>,
    Json(payload): Json<WriteNote>,
) -> Result<(StatusCode, Json<Note>)> {
    match payload.validate() {
        Err(e) => return Err((StatusCode::BAD_REQUEST, Json(e)).into()),
        _ => (),
    };

    let note: Note = Note {
        id,
        title: String::from(payload.title.trim()),
        body: String::from(payload.body.trim()),
    };

    let res: MySqlQueryResult = match sqlx::query(
        "UPDATE Notes SET title=AES_ENCRYPT(?, ?), body=AES_ENCRYPT(?, ?) WHERE id=?;",
    )
    .bind(&note.title)
    .bind(&aes_key)
    .bind(&note.body)
    .bind(&aes_key)
    .bind(&note.id)
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
        1 => Ok((StatusCode::OK, Json(note))),
        _ => Err(StatusCode::NOT_FOUND.into()),
    }
}

async fn delete_note(State(db_pool): State<Pool<MySql>>, Path(id): Path<u64>) -> StatusCode {
    let res: MySqlQueryResult = match sqlx::query("DELETE FROM Notes WHERE id=?;")
        .bind(id)
        .execute(&db_pool)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{e}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    match res.rows_affected() {
        1 => StatusCode::OK,
        _ => StatusCode::NOT_FOUND,
    }
}
