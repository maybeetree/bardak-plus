use sqlite::ConnectionThreadSafe;

use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;
use warp::http::StatusCode;
use warp::reply::with_status;

//use crate::schema;
use crate::db;
//use crate::auth;

pub struct App {
    //pub api_key: String,
    pub conn: ConnectionThreadSafe
}

impl App {
    pub async fn run(
            self: Arc<Self>
        ) {
        let app = self.clone();

        warp::serve(
            warp::path::end()
                .and_then(move || {
                    let app = app.clone();
                    async move { app.hello().await }
                })
        )
        .run(([0, 0, 0, 0], 3030))
        .await;
    }

    async fn hello(&self) -> Result<String, warp::Rejection> {
        Ok(
            format!(
                "Hello! I am {} version {}. ",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
            ).to_string()
        )
    }
}

