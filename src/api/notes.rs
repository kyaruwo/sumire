use axum::{
    extract::{DefaultBodyLimit, Path, State},
    http::StatusCode,
    response::Result,
    routing::{delete, get, post, put},
    Json, Router,
};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, FromRow, Pool, Postgres};
use uuid::Uuid;
use validator::{Validate, ValidationError};

pub fn routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/notes", post(write_note))
        .route("/notes", get(read_notes))
        .route("/notes/:note_id", get(read_note))
        .route("/notes/:note_id", put(update_note))
        .route("/notes/:note_id", delete(delete_note))
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
    note_id: Uuid,
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
        note_id: Uuid::new_v4(),
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
    .bind(note.note_id)
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

    eprintln!("users > write_note > {error}");
    Err(StatusCode::INTERNAL_SERVER_ERROR.into())
}

async fn read_notes(
    State(pool): State<Pool<Postgres>>,
    cookies: CookieJar,
) -> Result<Json<Vec<Note>>> {
    let session_id: &str = match cookies.get("session_id") {
        Some(cookie) => cookie.value(),
        None => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    let user_id: Uuid = match get_user_id(session_id, &pool).await {
        Err(e) => return Err(e),
        Ok(user_id) => user_id,
    };

    let notes: Vec<Note> = match sqlx::query_as::<_, Note>(
        "
    SELECT
        NOTE_ID,
        TITLE,
        BODY
    FROM
        NOTES
    WHERE
        USER_ID = $1;
    ",
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("notes > read_notes > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    Ok(Json(notes))
}

async fn read_note(
    State(pool): State<Pool<Postgres>>,
    cookies: CookieJar,
    Path(note_id): Path<Uuid>,
) -> Result<Json<Note>> {
    let session_id: &str = match cookies.get("session_id") {
        Some(cookie) => cookie.value(),
        None => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    let user_id: Uuid = match get_user_id(session_id, &pool).await {
        Err(e) => return Err(e),
        Ok(user_id) => user_id,
    };

    let res: Option<Note> = match sqlx::query_as::<_, Note>(
        "
    SELECT
        NOTE_ID,
        TITLE,
        BODY
    FROM
        NOTES
    WHERE
        USER_ID = $1
        AND NOTE_ID = $2;
    ",
    )
    .bind(user_id)
    .bind(note_id)
    .fetch_optional(&pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("notes > read_note > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match res {
        Some(note) => Ok(Json(note)),
        None => Err(StatusCode::NOT_FOUND.into()),
    }
}

async fn update_note(
    State(pool): State<Pool<Postgres>>,
    cookies: CookieJar,
    Path(note_id): Path<Uuid>,
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
        note_id,
        title: String::from(payload.title.trim()),
        body: String::from(payload.body.trim()),
    };

    match sqlx::query(
        "
    UPDATE
        NOTES
    SET
        TITLE = $1,
        BODY = $2
    WHERE
        NOTE_ID = $3
        AND USER_ID = $4;
    ",
    )
    .bind(&note.title)
    .bind(&note.body)
    .bind(&note.note_id)
    .bind(user_id)
    .execute(&pool)
    .await
    {
        Ok(_) => Ok((StatusCode::CREATED, Json(note))),
        Err(e) => {
            eprintln!("notes > update_note > {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR.into())
        }
    }
}

async fn delete_note(
    State(pool): State<Pool<Postgres>>,
    cookies: CookieJar,
    Path(note_id): Path<Uuid>,
) -> Result<StatusCode> {
    let session_id: &str = match cookies.get("session_id") {
        Some(cookie) => cookie.value(),
        None => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    let user_id: Uuid = match get_user_id(session_id, &pool).await {
        Err(e) => return Err(e),
        Ok(user_id) => user_id,
    };

    let res: PgQueryResult = match sqlx::query(
        "
    DELETE FROM
        NOTES
    WHERE
        USER_ID = $1
        AND NOTE_ID = $2;
    ",
    )
    .bind(user_id)
    .bind(note_id)
    .execute(&pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("notes > delete_note > {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match res.rows_affected() {
        1 => Ok(StatusCode::OK),
        _ => Err(StatusCode::NOT_FOUND.into()),
    }
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
