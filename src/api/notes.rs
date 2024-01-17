use axum::{
    extract::{DefaultBodyLimit, State},
    http::StatusCode,
    response::Result,
    routing::{delete, get, post, put},
    Json, Router,
};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;
use validator::{Validate, ValidationError};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/notes", post(write_note))
        .route("/notes", get(read_notes))
        .route("/notes/:id", get(read_note))
        .route("/notes/:id", put(update_note))
        .route("/notes/:id", delete(delete_note))
        .layer(DefaultBodyLimit::max(690))
}

fn empty_string(field: &String) -> Result<(), ValidationError> {
    if field.trim().is_empty() {
        return Err(ValidationError::new("empty"));
    }
    Ok(())
}

#[derive(Deserialize, Validate)]
struct WriteNote {
    #[validate(
        custom(function = "empty_string", message = "empty_title"),
        length(max = 42, message = "length_title")
    )]
    title: String,
    #[validate(
        custom(function = "empty_string", message = "empty_body"),
        length(max = 420, message = "length_body")
    )]
    body: String,
}

#[derive(Serialize, Deserialize, FromRow, Validate)]
struct Note {
    id: Uuid,
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
    State(pool): State<Pool<Postgres>>,
    cookies: CookieJar,
    Json(payload): Json<WriteNote>,
) -> Result<(StatusCode, Json<Note>)> {
    let session_id: &str = match cookies.get("session_id") {
        Some(cookie) => cookie.value(),
        None => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    let user_id: Uuid = match get_user_id(session_id, &pool).await {
        Err(e) => return Err(e),
        Ok(user_id) => user_id,
    };

    match payload.validate() {
        Err(e) => return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(e)).into()),
        _ => (),
    };

    let note: Note = Note {
        id: Uuid::new_v4(),
        title: String::from(payload.title.trim()),
        body: String::from(payload.body.trim()),
    };

    let error: sqlx::Error = match sqlx::query(
        "
    INSERT INTO
        NOTES (NOTE_ID, USER_ID, TITLE, BODY, CREATED_AT)
    VALUES
        ($1, $2, $3, $4, $5);
    ",
    )
    .bind(note.id)
    .bind(user_id)
    .bind(&note.title)
    .bind(&note.body)
    .bind(Utc::now())
    .execute(&pool)
    .await
    {
        Ok(_) => return Ok((StatusCode::CREATED, Json(note))),
        Err(e) => e,
    };

    match error.as_database_error() {
        Some(e) => {
            if e.constraint() == Some("notes_pkey") {
                return Err((StatusCode::CONFLICT, "UUID").into());
            }
        }
        None => (),
    }

    eprintln!("users > register > {error}");
    Err(StatusCode::INTERNAL_SERVER_ERROR.into())
}

async fn read_notes() {
    todo!()
}

async fn read_note() {
    todo!()
}

async fn update_note() {
    todo!()
}

async fn delete_note() {
    todo!()
}

#[derive(FromRow)]
struct UserID {
    user_id: Uuid,
}

async fn get_user_id(session_id: &str, pool: &Pool<Postgres>) -> Result<Uuid> {
    let res: Option<UserID> = match sqlx::query_as::<_, UserID>(
        "
        SELECT
            USER_ID
        FROM
            USERS
        WHERE
            SESSION_ID = $1
            AND VERIFIED = TRUE;
        ",
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("notes > get_user_id > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match res {
        Some(user) => Ok(user.user_id),
        None => Err(StatusCode::UNAUTHORIZED.into()),
    }
}
