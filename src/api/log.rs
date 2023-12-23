use sqlx::{MySql, Pool};

pub async fn new(user_id: u64, action: &str, db_pool: &Pool<MySql>) {
    match sqlx::query(
        "
        INSERT INTO
            Logs (`user_id`, `action`)
        VALUES
            (?, ?);
        ",
    )
    .bind(user_id)
    .bind(action)
    .execute(db_pool)
    .await
    {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    }
}
