use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, FromRow, MySql, Pool};
use validator::{Validate, ValidationError};

pub fn routes() -> Router<Pool<MySql>, Body> {
    Router::new()
        .route("/notes", post(write_note))
        .route("/notes", get(read_notes))
        .route("/notes/:id", put(update_note))
        .route("/notes/:id", delete(delete_note))
        .layer(DefaultBodyLimit::max(420))
}

#[derive(Deserialize, Validate)]
struct WriteNote {
    #[validate(custom = "empty_field")]
    title: String,
    #[validate(custom = "empty_field")]
    body: String,
}

#[derive(Serialize, Deserialize, FromRow, Validate)]
struct Note {
    id: u64,
    #[validate(custom = "empty_field")]
    title: String,
    #[validate(custom = "empty_field")]
    body: String,
}

fn empty_field(field: &String) -> Result<(), ValidationError> {
    if field.trim().is_empty() {
        return Err(ValidationError::new("empty_field"));
    }
    Ok(())
}

async fn write_note(
    State(pool): State<Pool<MySql>>,
    Json(payload): Json<WriteNote>,
) -> Result<(StatusCode, Json<Note>), (StatusCode, String)> {
    match payload.validate() {
        Err(e) => return Err((StatusCode::BAD_REQUEST, e.to_string())),
        _ => (),
    };

    let id: u64 = match sqlx::query("INSERT INTO Notes (title, body) values (?, ?);")
        .bind(&payload.title)
        .bind(&payload.body)
        .execute(&pool)
        .await
    {
        Ok(res) => res.last_insert_id(),
        Err(e) => {
            eprintln!("{e}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, String::new()));
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
) -> Result<StatusCode, (StatusCode, String)> {
    match payload.validate() {
        Err(e) => return Err((StatusCode::BAD_REQUEST, e.to_string())),
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
            return Err((StatusCode::INTERNAL_SERVER_ERROR, String::new()));
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
