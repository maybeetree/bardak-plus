use sqlx::sqlite::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteQueryResult;
use std::env;
use std::collections::HashMap;
use itertools::Itertools;

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


pub async fn get_latest_rows(
        pool: &SqlitePool,
        payload: &schema::GetLatestRows,
        ) -> Result<schema::ResponseGetLatestRows, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
SELECT item.id AS id, attr.val AS attr_val, attr.name AS attr_name
FROM item
INNER JOIN item_attr ON item.id = item_attr.item_id
INNER JOIN attr ON attr.name = item_attr.attr_name
	AND attr.val = item_attr.attr_val
ORDER BY item.created
LIMIT ?1 OFFSET ?2
;
"#,
    payload.limit,
    payload.offset
    )
    .fetch_all(pool)
    .await?;

    // TODO pagination is more difficult than this
    // e.g. cursors to manage mutating data and large offsets

    Ok(schema::ResponseGetLatestRows{
        rows: rows.into_iter().map(
            |row| { schema::ResponseGetLatestRowsInner {
                item_id: row.id
                    .expect("id should not be null"),
                attr_name: row.attr_name
                    .expect("attr_name should not be null"),
                attr_val: row.attr_val
                    .expect("attr_val should not be null"),
            } } // TODO check unwraps
            ).collect()
    })
}

pub async fn get_latest_items(
        pool: &SqlitePool,
        payload: &schema::GetLatestItems,
        ) -> Result<schema::ResponseGetLatestItems, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
SELECT item.id AS item_id, attr.val AS attr_val, attr.name AS attr_name
FROM item
INNER JOIN item_attr ON item.id = item_attr.item_id
INNER JOIN attr ON attr.name = item_attr.attr_name
	AND attr.val = item_attr.attr_val
ORDER BY item.created
LIMIT ?1 OFFSET ?2
;
"#,
    payload.limit,
    payload.offset
    )
    .fetch_all(pool)
    .await?;

    // TODO fix copypasta
    
    // TODO assume that correct group by item?
    // or
    // TODO preallocate??

    Ok(schema::ResponseGetLatestItems{
        items:
            rows
            .into_iter()
            .chunk_by(
                |row| row.item_id.expect("id should not be null")
                )
            .into_iter()
            .map(
                |(item_id, group)| {
                    schema::ResponseGetLatestItemsInner {
                        item_id: item_id,
                        attrs:
                            group
                            .into_iter()
                            .map(|row| (
                                    row.attr_name
                                        .expect("attr name != null"),
                                    row.attr_val
                                        .expect("attr value != null")
                                    ))
                            .collect()
                    }
                }
            ).collect()
    })
}
