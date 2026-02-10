
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // warp logging

    let api_service =
        OpenApiService::new(Api::new().await, "Hello World", "1.0")
        .server("http://localhost:3030");
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/", api_service)
        .nest("/docs", ui);

    Server::new(TcpListener::bind("0.0.0.0:3030"))
        .run(app)
        .await;

}

