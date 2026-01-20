use sqlx::sqlite::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteQueryResult;
use std::env;

use crate::schema;

pub async fn get_db(filename: &str) -> Result<SqlitePool, sqlx::Error> {
    //let options = SqliteConnectOptions::new()
    //    .filename(filename)
    //    .create_if_missing(true);

    //let pool = SqlitePool::connect_with(options).await?;
    //init_db(&pool).await?;
    //TODO
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap()).await?;
    Ok(pool)
}

const INIT_QUERY: &str = include_str!("../migrations/20240430231622_init.up.sql");

pub async fn init_db(pool: &SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query(
        INIT_QUERY
    )
        .execute(&mut *conn)
        .await
}


pub async fn get_latest(
        pool: &SqlitePool,
        payload: &schema::GetLatest,
        ) -> Result<schema::ResponseGetLatest, sqlx::Error> {
    let top = sqlx::query!(
        r#"SELECT id, created FROM item"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(schema::ResponseGetLatest{
        top: top.into_iter().map(
            |row| { schema::ResponseGetLatestInner { id: row.id } }
            ).collect()
    })
}
