use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, FromRow, MySql, Pool};
use std::borrow::Cow;
use validator::{Validate, ValidationError};

pub fn routes() -> Router<Pool<MySql>, Body> {
    Router::new()
        .route("/notes", post(write_note))
        .route("/notes/:id", get(read_note))
        .route("/notes", get(read_notes))
        .route("/notes/:id", put(update_note))
        .route("/notes/:id", delete(delete_note))
        .layer(DefaultBodyLimit::max(420))
}

#[derive(Deserialize, Validate)]
struct WriteNote {
    #[validate(custom = "empty_string")]
    title: String,
    #[validate(custom = "empty_string")]
    body: String,
}

#[derive(Serialize, Deserialize, FromRow, Validate)]
struct Note {
    id: u64,
    #[validate(custom = "empty_string")]
    title: String,
    #[validate(custom = "empty_string")]
    body: String,
}

fn empty_string(field: &String) -> Result<(), ValidationError> {
    if field.trim().is_empty() {
        let mut val_err: ValidationError = ValidationError::new("empty");
        val_err.message = Some(Cow::from("empty_string"));
        return Err(val_err);
    }
    Ok(())
}

async fn write_note(
    State(pool): State<Pool<MySql>>,
    Json(payload): Json<WriteNote>,
) -> Result<(StatusCode, Json<Note>), Response> {
    match payload.validate() {
        Err(e) => return Err(Json(e).into_response()),
        _ => (),
    };

    let mut note: Note = Note {
        id: 0,
        title: payload.title,
        body: payload.body,
    };

    note.id = match sqlx::query("INSERT INTO Notes (title, body) values (?, ?);")
        .bind(&note.title)
        .bind(&note.body)
        .execute(&pool)
        .await
    {
        Ok(res) => res.last_insert_id(),
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    };

    Ok((StatusCode::CREATED, Json(note)))
}

async fn read_note(
    State(pool): State<Pool<MySql>>,
    Path(id): Path<u64>,
) -> Result<Json<Note>, StatusCode> {
    let res: Option<Note> = match sqlx::query_as::<_, Note>("SELECT * FROM Notes WHERE id=?;")
        .bind(id)
        .fetch_optional(&pool)
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
        None => return Err(StatusCode::NOT_FOUND),
    }
}

async fn read_notes(State(pool): State<Pool<MySql>>) -> Result<Json<Vec<Note>>, StatusCode> {
    let notes: Vec<Note> = match sqlx::query_as::<_, Note>("SELECT * FROM Notes;")
        .fetch_all(&pool)
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
    State(pool): State<Pool<MySql>>,
    Path(id): Path<u64>,
    Json(payload): Json<WriteNote>,
) -> Result<StatusCode, Response> {
    match payload.validate() {
        Err(e) => return Err(Json(e).into_response()),
        _ => (),
    };

    let res: MySqlQueryResult = match sqlx::query("UPDATE Notes SET title=?, body=? WHERE id=?;")
        .bind(payload.title)
        .bind(payload.body)
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    };
    match res.rows_affected() {
        1 => Ok(StatusCode::OK),
        _ => Ok(StatusCode::NOT_FOUND),
    }
}

async fn delete_note(State(pool): State<Pool<MySql>>, Path(id): Path<u64>) -> StatusCode {
    let res: MySqlQueryResult = match sqlx::query("DELETE FROM Notes WHERE id=?;")
        .bind(id)
        .execute(&pool)
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
