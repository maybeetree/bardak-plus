use sqlx::sqlite::SqlitePool;

use crate::config::Config;
use crate::db;
//use crate::api::Api;
use std::fs::create_dir_all;
use anyhow::Result;



pub struct State {
    pub pool: SqlitePool,
}

impl State {
    pub async fn new(config: &Config) -> Result<Self> {
        create_dir_all(&config.media_upload_dir)?;
        create_dir_all(&config.media_save_dir)?;

        Ok(Self {
            pool: db::get_db(config.database.as_str())
                .await
                .expect("Failed to init db"),
        })
    }
}

