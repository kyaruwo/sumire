use axum::{extract, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, Pool};

#[derive(Deserialize)]
pub struct CreateNote {
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Note {
    id: u64,
    title: String,
    body: String,
}

pub async fn create_note(
    extract::State(pool): extract::State<Pool<MySql>>,
    Json(payload): Json<CreateNote>,
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
    return Ok((
        StatusCode::CREATED,
        Json(Note {
            id,
            title: payload.title,
            body: payload.body,
        }),
    ));
}

pub async fn read_notes(
    extract::State(pool): extract::State<Pool<MySql>>,
) -> Result<Json<Vec<Note>>, StatusCode> {
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
    return Ok(Json(notes));
}

pub async fn update_note() {}

pub async fn delete_note() {}
