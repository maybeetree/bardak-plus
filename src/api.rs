use poem_openapi::OpenApi;
use poem_openapi::payload::PlainText;
use poem_openapi::payload::Json;

use poem_openapi::param::Query;
//use poem::web::Query;

use crate::schema;
use crate::schema::DBResponse;
use crate::schema::DBError;
use crate::schema::ResGetLatestRows;
use crate::schema::ResponseGetLatestItems;
//use crate::schema::ReqGetLatestRows;
use crate::db;
use crate::state::State;
use std::sync::Arc;

pub struct Api {
    state: Arc<State>,
}

#[OpenApi]
impl Api {
    pub async fn new() -> Self {
        Self {
            state: Arc::new(State::new().await),
        }
    }

    /// Hello!
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<String> {
        PlainText(
            format!(
                "Hello! I am {} version {}. \
                I am licensed under {}, \
                and my source code is at {}.",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_LICENSE"),
                env!("CARGO_PKG_REPOSITORY"),
            )
        )
    }

    /// Get latest entries (by row)
    #[oai(path = "/latest-rows", method = "get")]
    async fn latest_rows(
            &self,
            //payload: Query<ReqGetLatestRows>,
            #[oai(default = "schema::default_limit")] limit: Query<i64>,
            #[oai(default = "schema::default_offset")] offset: Query<i64>,
            ) -> DBResponse<ResGetLatestRows> {

        match db::latest_rows(&self.state.pool, *limit, *offset).await {
            Ok(v) => DBResponse::Ok(Json(v)),
            Err(e) => DBResponse::Error(
                Json(DBError {error: e.to_string()})
            ),
        }
    }

    /// Get latest items
    #[oai(path = "/latest-items", method = "get")]
    async fn latest_items(
            &self,
            //payload: Query<ReqGetLatestRows>,
            #[oai(default = "schema::default_limit")] limit: Query<i64>,
            #[oai(default = "schema::default_offset")] offset: Query<i64>,
            ) -> DBResponse<ResponseGetLatestItems> {

        match db::latest_items(&self.state.pool, *limit, *offset).await {
            Ok(v) => DBResponse::Ok(Json(v)),
            Err(e) => DBResponse::Error(
                Json(DBError {error: e.to_string()})
            ),
        }
    }
}

