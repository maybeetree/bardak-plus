use sqlx::sqlite::SqlitePool;

use std::sync::Arc;
use crate::config::Config;
use crate::db;
//use crate::api::Api;
use crate::api;
use conf::Conf;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};


pub struct State {
    pub pool: SqlitePool,
}

impl State {
    pub async fn new() -> Self {
        let config = Config::parse();

        Self {
            pool: db::get_db(config.database.as_str())
                .await
                .expect("Failed to init db"),
        }
    }
}

