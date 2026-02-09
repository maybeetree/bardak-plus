use sqlx::sqlite::SqlitePool;

use crate::config::Config;
use crate::db;
//use crate::api::Api;
use conf::Conf;



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

