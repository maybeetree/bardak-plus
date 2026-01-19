use std::sync::Arc;
use std::env;

use crate::app::App;
mod app;
mod db;

#[tokio::main]
async fn main() {
    let app = Arc::new(
        App {
            conn: db::get_db().expect("Failed to init db")
            }
        );

    app.run().await;
    //api::run(conn).await;
}


