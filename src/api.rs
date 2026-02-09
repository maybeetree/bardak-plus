use poem_openapi::OpenApi;
use poem_openapi::payload::PlainText;
use poem_openapi::payload::Attachment;
use poem_openapi::payload::AttachmentType;
use poem_openapi::payload::Json;
use poem_openapi::types::ToJSON;
use poem_openapi::ApiResponse;
use poem::IntoResponse;

use poem_openapi::param::Query;
//use poem::web::Query;

use crate::schema;
use crate::schema::DBResponse;
use crate::schema::BinResponse;
use crate::schema::Error;
use crate::schema::ResLatestRows;
use crate::schema::ResLatestItems;
//use crate::schema::ReqGetLatestRows;
use crate::db;
use crate::state::State;
use std::sync::Arc;

fn into_db_response<T: ToJSON>(
        result: Result<T, sqlx::Error>,
    ) -> DBResponse<T> {
    match result {
        Ok(v) => DBResponse::Ok(Json(v)),
        Err(e) => DBResponse::Error(Json(Error { error: e.to_string() })),
    }
}

static SOURCE_ARCHIVE: &'static [u8] = include_bytes!(env!("SOURCE_ARCHIVE"));

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

    /// Information endpoint
    ///
    /// This method returns some basic information about
    /// the bardak server and provides a link to the source code
    /// repository
    /// (as required by the AGPL license)
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

    /// Source code archive
    ///
    /// Returns a tarball of the source code
    /// that this bardak server is running
    #[oai(path = "/source", method = "get")]
    async fn get_source(&self)
            -> BinResponse
            {
        BinResponse::Ok(
            Attachment::<&'static [u8]>::new(SOURCE_ARCHIVE)
                .filename("bardak.tar.gz") // TODO git tag or smt
                // tell browsers to download it as a file
                // instead of trying to display it
                .attachment_type(AttachmentType::Attachment)
            )
    }

    /// Get latest entries (by row)
    #[oai(path = "/latest-rows", method = "get")]
    async fn latest_rows(
            &self,
            //payload: Query<ReqLatestRows>,
            #[oai(default = "schema::default_limit")] limit: Query<i64>,
            #[oai(default = "schema::default_offset")] offset: Query<i64>,
            ) -> DBResponse<ResLatestRows> {

        into_db_response(
            db::latest_rows(
                &self.state.pool,
                *limit,
                *offset
                ).await
            )
        }

    /// Get latest items
    #[oai(path = "/latest-items", method = "get")]
    async fn latest_items(
            &self,
            //payload: Query<ReqLatestRows>,
            #[oai(default = "schema::default_limit")] limit: Query<i64>,
            #[oai(default = "schema::default_offset")] offset: Query<i64>,
            ) -> DBResponse<ResLatestItems> {

        into_db_response(
            db::latest_items(
                &self.state.pool,
                *limit,
                *offset
                ).await
            )
    }
}

