use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use sqlx::ConnectOptions;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::migrate::MigrateError;
use itertools::Itertools;
use tracing::log::LevelFilter;

use crate::schema;

pub async fn get_db(filename: &str) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .log_statements(LevelFilter::Info)
        .create_if_missing(true)
        ;

    let pool = SqlitePool::connect_with(options)
        .await?;
    init_db(&pool).await?;
    Ok(pool)
}

pub async fn init_db(pool: &SqlitePool) -> Result<(), MigrateError> {
    sqlx::migrate!()
        .run(pool)
        .await
}

pub async fn latest_rows(
    pool: &SqlitePool,
    //payload: &schema::ReqetLatestRows,
    limit: i64,
    offset: i64,
) -> Result<schema::ResLatestRows, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT
            item.id        AS id,
            attr.val       AS attr_val,
            attr.name      AS attr_name
        FROM item
        INNER JOIN item_attr
            ON item.id = item_attr.item_id
        INNER JOIN attr
            ON attr.name = item_attr.attr_name
           AND attr.val  = item_attr.attr_val
        ORDER BY item.created
        LIMIT ?1 OFFSET ?2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(schema::ResLatestRows {
        rows: rows
            .into_iter()
            .map(|row| schema::ResLatestRowsInner {
                item_id: row
                    .try_get::<i64, _>("id")
                    .expect("id should not be null"),
                attr_name: row
                    .try_get::<String, _>("attr_name")
                    .expect("attr_name should not be null"),
                attr_val: row
                    .try_get::<String, _>("attr_val")
                    .expect("attr_val should not be null"),
            })
            .collect(),
    })
    // TODO expects 
}

pub async fn latest_items(
        pool: &SqlitePool,
        //payload: &schema::GetLatestItems,
        limit: i64,
        offset: i64,
        ) -> Result<schema::ResLatestItems, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        WITH litem AS (
            SELECT id
            FROM item
            ORDER BY created
            LIMIT ?1 OFFSET ?2
        )
        SELECT
            id AS item_id,
            attr.val AS attr_val,
            attr.name AS attr_name
            FROM litem
        INNER JOIN item_attr ON litem.id = item_attr.item_id
        INNER JOIN attr ON attr.name = item_attr.attr_name
                AND attr.val = item_attr.attr_val
        ORDER BY item_attr.item_id, attr.name;
        ;
        "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    // TODO assume that correct group by item?
    // or
    // TODO preallocate??

    Ok(schema::ResLatestItems{
        items:
            rows
            .into_iter()
            .chunk_by(
                |row| row
                    .try_get::<i64, _>("item_id")
                    .expect("id should not be null")
                )
            .into_iter()
            .map(
                |(item_id, group)| {
                    schema::ResLatestItemsInner {
                        item_id: item_id,
                        attrs:
                            group
                            .into_iter()
                            .map(|row| (
                                    row
                                        .try_get::<String, _>("attr_name")
                                        .expect("attr name != null"),
                                    row
                                        .try_get::<String, _>("attr_val")
                                        .expect("attr value != null")
                                    ))
                            .collect()
                    }
                }
            ).collect()
    })
}

pub async fn add_item(
        pool: &SqlitePool,
        payload: &schema::ReqAddItem,
        ) -> Result<schema::ResAddItem, sqlx::Error>
{
    let mut tx = pool.begin().await?;

    sqlx::query(r#"INSERT INTO item DEFAULT VALUES;"#)
        .execute(&mut *tx)
        .await?;

    let item_id = sqlx::query_scalar("SELECT last_insert_rowid();")
        .fetch_one(&mut *tx)
        .await?;

    for (attr_name, attr_val) in &payload.attrs
    {
        sqlx::query("
            INSERT INTO attr (name, val)
            VALUES (?, ?);
            ")
            .bind(&attr_name)
            .bind(&attr_val)
            .execute(&mut *tx)
            .await?;

        sqlx::query("
            INSERT INTO item_attr (item_id, attr_name, attr_val)
            VALUES (?, ?, ?);
            ")
            .bind(item_id)
            .bind(attr_name)
            .bind(attr_val)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    Ok(schema::ResAddItem {
        item_id: item_id
    })
}

pub async fn add_media(
        pool: &SqlitePool,
        payload: Vec<u8>,
        ) -> Result<schema::ResAddMedia, sqlx::Error>
{

    Ok(schema::ResAddMedia {
        media_id: 69
    })
}

