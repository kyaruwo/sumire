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

#[derive(Serialize)]
struct Response {
    reason: String,
}

fn server_error() -> (StatusCode, Json<Response>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(Response {
            reason: String::from("SERVER_ERROR"),
        }),
    )
}

fn empty_field(field: &str) -> (StatusCode, Json<Response>) {
    (
        StatusCode::BAD_REQUEST,
        Json(Response {
            reason: format!("EMPTY {field}"),
        }),
    )
}

#[derive(Deserialize)]
struct WriteNote {
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize, FromRow)]
struct Note {
    id: u64,
    title: String,
    body: String,
}

async fn write_note(
    State(pool): State<Pool<MySql>>,
    Json(payload): Json<WriteNote>,
) -> Result<(StatusCode, Json<Note>), (StatusCode, Json<Response>)> {
    let mut note: Note = Note {
        id: 0,
        title: String::from(payload.title.trim()),
        body: String::from(payload.body.trim()),
    };

    // validation
    if note.title.is_empty() {
        return Err(empty_field("title"));
    }
    if note.body.is_empty() {
        return Err(empty_field("body"));
    }

    // write note and get id
    note.id = match sqlx::query("INSERT INTO Notes (title, body) values (?, ?);")
        .bind(&note.title)
        .bind(&note.body)
        .execute(&pool)
        .await
    {
        Ok(res) => res.last_insert_id(),
        Err(e) => {
            eprintln!("{e}");
            return Err(server_error());
        }
    };

    // response
    Ok((StatusCode::CREATED, Json(note)))
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
) -> Result<StatusCode, (StatusCode, Json<Response>)> {
    let note: Note = Note {
        id,
        title: String::from(payload.title.trim()),
        body: payload.body,
    };

    // validation
    if note.title.is_empty() {
        return Err(empty_field("title"));
    }
    if note.body.is_empty() {
        return Err(empty_field("body"));
    }

    let res: MySqlQueryResult = match sqlx::query("UPDATE Notes SET title=?, body=? WHERE id=?;")
        .bind(note.title)
        .bind(note.body)
        .bind(note.id)
        .execute(&pool)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{e}");
            return Err(server_error());
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
