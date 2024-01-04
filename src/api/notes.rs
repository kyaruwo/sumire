use axum::{
    extract::{DefaultBodyLimit, Path, State},
    http::StatusCode,
    response::Result,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlQueryResult, FromRow, MySql, Pool};
use validator::{Validate, ValidationError};

pub fn routes() -> Router<Pool<MySql>> {
    Router::new()
        .route("/notes", post(write_note))
        .route("/notes/:id", get(read_note))
        .route("/notes", get(read_notes))
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
    cookies: CookieJar,
    Json(payload): Json<WriteNote>,
) -> Result<(StatusCode, Json<Note>)> {
    let token: &str = match cookies.get("token") {
        Some(cookie) => cookie.value(),
        None => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    let user_id: u64 = match get_user_id(token, &aes_key, &db_pool).await {
        Err(e) => return Err(e),
        Ok(user_id) => user_id,
    };

    match payload.validate() {
        Err(e) => return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(e)).into()),
        _ => (),
    };

    let mut note: Note = Note {
        id: 0,
        title: String::from(payload.title.trim()),
        body: String::from(payload.body.trim()),
    };

    note.id = match sqlx::query(
        "
        INSERT INTO
            Notes (`user_id`, `title`, `body`)
        VALUES
            (?, AES_ENCRYPT(?, ?), AES_ENCRYPT(?, ?));
        ",
    )
    .bind(user_id)
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

    log(user_id, "write_note", note.id, "created", &db_pool).await;
    Ok((StatusCode::CREATED, Json(note)))
}

async fn read_note(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
    Path(note_id): Path<u64>,
    cookies: CookieJar,
) -> Result<Json<Note>> {
    let token: &str = match cookies.get("token") {
        Some(cookie) => cookie.value(),
        None => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    let user_id: u64 = match get_user_id(token, &aes_key, &db_pool).await {
        Err(e) => return Err(e),
        Ok(user_id) => user_id,
    };

    let res: Option<Note> = match sqlx::query_as::<_, Note>(
        "
        SELECT
            id,
            CONVERT(AES_DECRYPT(title, ?) USING utf8) as `title`,
            CONVERT(AES_DECRYPT(body, ?) USING utf8) as `body`
        FROM
            Notes
        WHERE
            `user_id` = ?
            AND id = ?;
        ",
    )
    .bind(&aes_key)
    .bind(&aes_key)
    .bind(user_id)
    .bind(note_id)
    .fetch_optional(&db_pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match res {
        Some(note) => {
            log(user_id, "read_note", note_id, "ok", &db_pool).await;
            Ok(Json(note))
        }
        None => {
            log(user_id, "read_note", note_id, "not_found", &db_pool).await;
            Err(StatusCode::NOT_FOUND.into())
        }
    }
}

async fn read_notes(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
    cookies: CookieJar,
) -> Result<Json<Vec<Note>>> {
    let token: &str = match cookies.get("token") {
        Some(cookie) => cookie.value(),
        None => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    let user_id: u64 = match get_user_id(token, &aes_key, &db_pool).await {
        Err(e) => return Err(e),
        Ok(user_id) => user_id,
    };

    let notes: Vec<Note> = match sqlx::query_as::<_, Note>(
        "
        SELECT
            id,
            CONVERT(AES_DECRYPT(title, ?) USING utf8) as `title`,
            CONVERT(AES_DECRYPT(body, ?) USING utf8) as `body`
        FROM
            Notes
        WHERE
            `user_id` = ?;
        ",
    )
    .bind(&aes_key)
    .bind(&aes_key)
    .bind(user_id)
    .fetch_all(&db_pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    log(user_id, "read_notes", 0, "ok", &db_pool).await;
    Ok(Json(notes))
}

async fn update_note(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
    Path(note_id): Path<u64>,
    cookies: CookieJar,
    Json(payload): Json<WriteNote>,
) -> Result<Json<Note>> {
    let token: &str = match cookies.get("token") {
        Some(cookie) => cookie.value(),
        None => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    let user_id: u64 = match get_user_id(token, &aes_key, &db_pool).await {
        Err(e) => return Err(e),
        Ok(user_id) => user_id,
    };

    match payload.validate() {
        Err(e) => return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(e)).into()),
        _ => (),
    };

    let note: Note = Note {
        id: note_id,
        title: String::from(payload.title.trim()),
        body: String::from(payload.body.trim()),
    };

    let res: MySqlQueryResult = match sqlx::query(
        "
        UPDATE
            Notes
        SET
            `title` = AES_ENCRYPT(?, ?),
            `body` = AES_ENCRYPT(?, ?)
        WHERE
            `user_id` = ?
            AND id = ?;
        ",
    )
    .bind(&note.title)
    .bind(&aes_key)
    .bind(&note.body)
    .bind(&aes_key)
    .bind(user_id)
    .bind(note.id)
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
        1 => {
            log(user_id, "update_note", note_id, "ok", &db_pool).await;
            Ok(Json(note))
        }
        _ => {
            log(user_id, "update_note", note_id, "not_found", &db_pool).await;
            Err(StatusCode::NOT_FOUND.into())
        }
    }
}

async fn delete_note(
    State(db_pool): State<Pool<MySql>>,
    Extension(aes_key): Extension<String>,
    Path(note_id): Path<u64>,
    cookies: CookieJar,
) -> Result<StatusCode> {
    let token: &str = match cookies.get("token") {
        Some(cookie) => cookie.value(),
        None => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    let user_id: u64 = match get_user_id(token, &aes_key, &db_pool).await {
        Err(e) => return Err(e),
        Ok(user_id) => user_id,
    };

    let res: MySqlQueryResult = match sqlx::query(
        "
        DELETE FROM
            Notes
        WHERE
            `user_id` = ?
            AND id = ?;
        ",
    )
    .bind(user_id)
    .bind(note_id)
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
        1 => {
            log(user_id, "delete_note", note_id, "ok", &db_pool).await;
            Ok(StatusCode::OK)
        }
        _ => {
            log(user_id, "delete_note", note_id, "not_found", &db_pool).await;
            Err(StatusCode::NOT_FOUND.into())
        }
    }
}

async fn log(user_id: u64, request: &str, note_id: u64, response: &str, db_pool: &Pool<MySql>) {
    match sqlx::query(
        "
        INSERT INTO
            NotesLogs (`user_id`, `request`, `note_id`, `response`)
        VALUES
            (?, ?, ?, ?);
        ",
    )
    .bind(user_id)
    .bind(request)
    .bind(note_id)
    .bind(response)
    .execute(db_pool)
    .await
    {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    }
}

#[derive(FromRow)]
struct UserID {
    id: u64,
}

async fn get_user_id(token: &str, aes_key: &String, db_pool: &Pool<MySql>) -> Result<u64> {
    let res: Option<UserID> = match sqlx::query_as::<_, UserID>(
        "
        SELECT
            id
        FROM
            Users
        WHERE
            `token` = AES_ENCRYPT(?, ?);
        ",
    )
    .bind(token)
    .bind(aes_key)
    .fetch_optional(db_pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
        }
    };

    match res {
        Some(user) => Ok(user.id),
        None => Err(StatusCode::UNAUTHORIZED.into()),
    }
}
