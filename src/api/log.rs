use sqlx::{MySql, Pool};

pub async fn new(user_id: u64, request: &str, response: &str, db_pool: &Pool<MySql>) {
    match sqlx::query(
        "
        INSERT INTO
            Logs (`user_id`, `request`, `response`)
        VALUES
            (?, ?, ?);
        ",
    )
    .bind(user_id)
    .bind(request)
    .bind(response)
    .execute(db_pool)
    .await
    {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    }
}
