
use anyhow::Result;
use poem::Route;
use poem::Server;
use poem::endpoint::StaticFilesEndpoint;
use poem::listener::TcpListener;
use poem_openapi::OpenApiService;
use tracing_subscriber; // warp logging

use crate::state::get_state;
use crate::config::get_config;
use crate::config::get_lconfig;

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


    let config = get_config().await;
    let lconfig = get_lconfig(&config).await?;
    let state = get_state(&config).await?;

    let api = Api::new_manual(&config, &lconfig, &state).await?;

    // TODO weird dance here with `config` let caller get it then pass it

    let thumbs = StaticFilesEndpoint::new(config.media_thumb_dir.clone())
        .show_files_listing()
        //.index_file("index.html")
        ;

    let api_service =
        OpenApiService::new(api, "Hello World", "1.0")
        .server("http://localhost:3030");

    let ui = api_service.swagger_ui();

    let app = Route::new()
        .nest("/", api_service)
        .nest("/docs", ui)
        .nest("/unstable/thumbs", thumbs) // TODO sync version with api??
        ;

    // TODO .nest for different version
    // TODO .data for passing pool/state instead of impl crutch?
    //      - actually no, impl I like better. Less boilerplate.

    Server::new(TcpListener::bind("0.0.0.0:3030"))
        .run(app)
        .await?;

    Ok(())
    // TODO infalliable??
}

