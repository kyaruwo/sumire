use axum::{extract, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, FromRow, MySql, Pool};

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
    extract::State(pool): extract::State<Pool<MySql>>,
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
    Ok(Json(notes))
}

pub async fn update_note(
    extract::State(pool): extract::State<Pool<MySql>>,
    extract::Path(id): extract::Path<u64>,
    Json(payload): Json<WriteNote>,
) -> StatusCode {
    let res: MySqlQueryResult = match sqlx::query("UPDATE Notes SET title=?, body=? WHERE id=?;")
        .bind(&payload.title)
        .bind(&payload.body)
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

pub async fn delete_note() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}
