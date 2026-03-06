use sqlx::sqlite::SqlitePool;

use crate::config::Config;
use crate::db;
//use crate::api::Api;
use std::fs::create_dir_all;
use anyhow::Result;
use tokio::sync::OnceCell;


static STATE: OnceCell<State> = OnceCell::const_new();

pub async fn get_state(config: &Config) -> Result<&'static State> {
    let state = State::new(&config).await?;
    Ok(STATE.get_or_init(async || { state } ).await)
}

pub struct State {
    pub pool: SqlitePool,
}

impl State {
    pub async fn new(config: &Config) -> Result<Self> {
        create_dir_all(&config.media_upload_dir)?;
        create_dir_all(&config.media_save_dir)?;
        create_dir_all(&config.media_thumb_dir)?;
        // TODO any way to ensure these get called using the type system??

        Ok(Self {
            pool: db::get_db(config.database.as_str())
                .await
                .expect("Failed to init db"),
        })
    }
}

