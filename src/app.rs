use sqlx::sqlite::SqlitePool;

use std::sync::Arc;
use crate::config::Config;
use crate::db;
//use crate::api::Api;
use crate::api;
use conf::Conf;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};


pub struct App {
    pub pool: SqlitePool,
}

impl App {
    pub async fn new() -> Self {
        let config = Config::parse();

        Self{
            pool: db::get_db(config.database.as_str())
                .await
                .expect("Failed to init db"),
        }
    }

    pub async fn run(self: Arc<Self>) {

        let api_service =
            OpenApiService::new(self, "Hello World", "1.0")
            .server("http://localhost:3030");
        let ui = api_service.swagger_ui();
        let app = Route::new()
            .nest("/", api_service)
            .nest("/docs", ui);

        Server::new(TcpListener::bind("0.0.0.0:3030"))
            .run(app)
            .await;

        //warp::serve(filters::root(self.clone()))
        //.run(([0, 0, 0, 0], 3030))
        //.await;
    }



}
