use axum::{extract, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

#[derive(Deserialize)]
pub struct CreateNote {
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
pub struct Note {
    id: u64,
    title: String,
    body: String,
}

pub async fn create_note(
    extract::State(pool): extract::State<Pool<MySql>>,
    Json(payload): Json<CreateNote>,
) -> Result<(StatusCode, Json<Note>), StatusCode> {
    let mut note: Note = Note {
        id: 0,
        title: payload.title,
        body: payload.body,
    };

    match sqlx::query("INSERT INTO Notes (title, body) values (?, ?);")
        .bind(&note.title)
        .bind(&note.body)
        .execute(&pool)
        .await
    {
        Ok(res) => {
            note.id = res.last_insert_id();
            return Ok((StatusCode::CREATED, Json(note)));
        }
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
}

pub async fn read_notes() {}

pub async fn update_note() {}

pub async fn delete_note() {}
