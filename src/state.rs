use sqlx::sqlite::SqlitePool;

use crate::config::Config;
use crate::db;
//use crate::api::Api;
use conf::Conf;
use std::fs::create_dir_all;
use anyhow::Result;



pub struct State {
    pub pool: SqlitePool,
    pub config: Config,
}

impl State {
    pub async fn new() -> Result<Self> {
        let config = Config::parse();

        create_dir_all(&config.media_upload_dir)?;
        create_dir_all(&config.media_save_dir)?;

        Ok(Self {
            pool: db::get_db(config.database.as_str())
                .await
                .expect("Failed to init db"),
            config: config,
        })
    }
}

