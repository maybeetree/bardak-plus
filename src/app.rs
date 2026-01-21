use sqlx::sqlite::SqlitePool;

use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;
use warp::reply::with_status;
use warp::http::StatusCode;
use crate::config::Config;
use crate::db;
use crate::schema;
use crate::filters;
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
        warp::serve(filters::root(self.clone()))
        .run(([0, 0, 0, 0], 3030))
        .await;
    }



}
