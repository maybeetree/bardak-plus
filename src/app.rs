use sqlite::ConnectionThreadSafe;

use std::sync::Arc;
use warp::Filter;

//use crate::schema;
//use crate::auth;

pub struct App {
    //pub api_key: String,
    pub conn: ConnectionThreadSafe,
}

impl App {
    pub async fn run(self: Arc<Self>) {
        let app = self.clone();

        warp::serve(warp::path::end().and_then(move || {
            let app = app.clone();
            async move { app.hello().await }
        }))
        .run(([0, 0, 0, 0], 3030))
        .await;
    }

    async fn hello(&self) -> Result<String, warp::Rejection> {
        Ok(format!(
            "Hello! I am {} version {}. ",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        )
        .to_string())
    }
}
