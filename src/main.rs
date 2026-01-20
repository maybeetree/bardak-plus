use std::sync::Arc;
use tracing_subscriber; // warp logging
use crate::app::App;
mod app;
mod db;
mod config;
mod schema;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // warp logging

    //let app = Arc::new(App {
    //    conn: db::get_db().expect("Failed to init db"),
    //});
    let app = Arc::new(App::new().await);

    app.run().await;
    //api::run(conn).await;
}

