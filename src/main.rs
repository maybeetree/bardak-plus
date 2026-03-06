
use anyhow::Result;
use poem::Route;
use poem::Server;
use poem::listener::TcpListener;
use poem_openapi::OpenApiService;
use tracing_subscriber; // warp logging

use crate::api::Api;
//use crate::app::App;
//mod app;
mod db;
mod config;
mod schema;
mod api;
mod state;
mod media;
mod tasks;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init(); // warp logging

    let api_service =
        OpenApiService::new(Api::new().await?, "Hello World", "1.0")
        .server("http://localhost:3030");
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/", api_service)
        .nest("/docs", ui);

    // TODO .nest for different version
    // TODO .data for passing pool/state instead of impl crutch?
    //      - actually no, impl I like better. Less boilerplate.

    Server::new(TcpListener::bind("0.0.0.0:3030"))
        .run(app)
        .await?;

    Ok(())
    // TODO infalliable??
}

