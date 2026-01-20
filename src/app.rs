use sqlx::sqlite::SqlitePool;

use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;
use warp::reply::with_status;
use warp::http::StatusCode;
use crate::config::Config;
use crate::db;
use crate::schema;
use conf::Conf;

pub struct App {
    //pub api_key: String,
    //pub conn: ConnectionThreadSafe,
    pub pool: SqlitePool,
}

impl App {
    pub async fn new() -> Self {
        let config = Config::parse();

        Self{
            pool: db::get_db(config.database.as_str())
                .await
                .expect("Failed to init db"),
        }
    }

    pub async fn run(self: Arc<Self>) {
        let app = self.clone();
        let app2 = self.clone();  // TODO
        let app3 = self.clone();  // TODO
        //println!("database file: {}", config.database);

        warp::serve(
            (
                warp::path::end().and_then(move || {
                    let app = app.clone();
                    async move { app.hello().await }
                })
            ).or(
                warp::path!("latest-rows")
                .and(warp::get())
                .and(warp::query::<schema::GetLatestRows>())
                .and_then(move |payload| {
                    let app = app2.clone();
                    async move { app.get_latest_rows(payload).await }
                })
            ).or(
                warp::path!("latest-items")
                .and(warp::get())
                .and(warp::query::<schema::GetLatestItems>())
                .and_then(move |payload| {
                    let app = app3.clone();
                    async move { app.get_latest_items(payload).await }
                })
            )
        )
        .run(([0, 0, 0, 0], 3030))
        .await;
    }

    async fn hello(&self) -> Result<String, warp::Rejection> {
        Ok(
            format!(
                "Hello! I am {} version {}. \
                I am licensed under {}, \
                and my source code is at {}.",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_LICENSE"),
                env!("CARGO_PKG_REPOSITORY"),
            ).to_string()
        )
    }

    pub async fn get_latest_rows(
            self: Arc<Self>,
            payload: schema::GetLatestRows,
        ) -> Result<Box<dyn warp::Reply>, Infallible> {

        // Here the response type is Box because
        // there are two possible responses
        // warp::reply::json and String
        // which have different sizes
        // so we need to have indirection.

        Ok(match db::get_latest_rows(&self.pool, &payload).await {
            Ok(v) => Box::new(with_status(
                warp::reply::json(&v),
                StatusCode::OK
                )),
            Err(e) => Box::new(with_status(
                format!("{:#?}", e),
                StatusCode::INTERNAL_SERVER_ERROR
                )),
        })
    }

    pub async fn get_latest_items(
            self: Arc<Self>,
            payload: schema::GetLatestItems,
        ) -> Result<Box<dyn warp::Reply>, Infallible> {

        // TODO fix copypasta

        Ok(match db::get_latest_items(&self.pool, &payload).await {
            Ok(v) => Box::new(with_status(
                warp::reply::json(&v),
                StatusCode::OK
                )),
            Err(e) => Box::new(with_status(
                format!("{:#?}", e),
                StatusCode::INTERNAL_SERVER_ERROR
                )),
        })
    }


}
