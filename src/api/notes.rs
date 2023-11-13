use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, FromRow, MySql, Pool};

pub fn routes() -> Router<Pool<MySql>, Body> {
    Router::new()
        .route("/notes", post(write_note))
        .route("/notes", get(read_notes))
        .route("/notes/:id", put(update_note))
        .route("/notes/:id", delete(delete_note))
        .layer(DefaultBodyLimit::max(420))
}

#[derive(Deserialize)]
pub struct WriteNote {
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Note {
    id: u64,
    title: String,
    body: String,
}

pub async fn write_note(
    State(pool): State<Pool<MySql>>,
    Json(payload): Json<WriteNote>,
) -> Result<(StatusCode, Json<Note>), StatusCode> {
    let id: u64 = match sqlx::query("INSERT INTO Notes (title, body) values (?, ?);")
        .bind(&payload.title)
        .bind(&payload.body)
        .execute(&pool)
        .await
    {
        Ok(res) => res.last_insert_id(),
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok((
        StatusCode::CREATED,
        Json(Note {
            id,
            title: payload.title,
            body: payload.body,
        }),
    ))
}

pub async fn read_notes(State(pool): State<Pool<MySql>>) -> Result<Json<Vec<Note>>, StatusCode> {
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

pub async fn update_note(
    State(pool): State<Pool<MySql>>,
    Path(id): Path<u64>,
    Json(payload): Json<WriteNote>,
) -> StatusCode {
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
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    match res.rows_affected() {
        1 => StatusCode::OK,
        _ => StatusCode::NOT_FOUND,
    }
}

pub async fn delete_note(State(pool): State<Pool<MySql>>, Path(id): Path<u64>) -> StatusCode {
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
