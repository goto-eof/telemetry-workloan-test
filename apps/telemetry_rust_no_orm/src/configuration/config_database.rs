use crate::{DB_POOL, SETTINGS};
use log::debug;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn establish_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&format!(
            "postgres://{}:{}@{}:{}/{}",
            SETTINGS.db_username.as_str(),
            SETTINGS.db_password.as_str(),
            SETTINGS.db_uri.as_str(),
            SETTINGS.db_port,
            SETTINGS.db_name.as_str()
        ))
        .await?;

    return Ok(pool);
}

pub async fn check_connection() -> () {
    let pool = DB_POOL.get().await;
    let row: Result<(i64,), sqlx::Error> = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(pool)
        .await;

    if row.is_ok() {
        debug!("[DB RESULT] DB Connection [OK]");
    } else {
        debug!("[DB RESULT] Connection to [DB FAILED]: {:?}", row.err());
    }
    ()
}
