use sqlite::ConnectionThreadSafe;

use std::sync::Arc;
use warp::Filter;
use crate::config::Config;
use conf::Conf;

pub struct App {
    //pub api_key: String,
    pub conn: ConnectionThreadSafe,
}

impl App {
    pub async fn run(self: Arc<Self>) {
        let app = self.clone();

        let config = Config::parse();
        println!("database file: {}", config.database);

        warp::serve(warp::path::end().and_then(move || {
            let app = app.clone();
            async move { app.hello().await }
        }))
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
}
