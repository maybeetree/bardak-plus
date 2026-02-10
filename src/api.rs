use poem_openapi::OpenApi;
use poem_openapi::payload::PlainText;
use poem_openapi::payload::Html;
use poem_openapi::payload::Attachment;
use poem_openapi::payload::AttachmentType;
use poem_openapi::payload::Json;
use poem_openapi::payload::Binary;
use poem_openapi::types::ToJSON;
use poem_openapi::ApiResponse;
use poem_openapi::payload::Form;
use poem::IntoResponse;

use poem_openapi::param::Query;
//use poem::web::Query;

use crate::schema;
use crate::schema::DBResponse;
use crate::schema::BinResponse;
use crate::schema::Error;
use crate::schema::ResLatestRows;
use crate::schema::ResLatestItems;
use crate::schema::ResAddItem;
use crate::schema::ReqAddItem;
use crate::schema::ResAddMedia;
//use crate::schema::ReqGetLatestRows;
use crate::db;
use crate::media;
use crate::state::State;
use std::sync::Arc;

use const_format::formatcp;

fn into_db_response<T: ToJSON>(
        result: Result<T, sqlx::Error>,
    ) -> DBResponse<T> {
    match result {
        Ok(v) => DBResponse::Ok(Json(v)),
        Err(e) => DBResponse::Error(Json(Error { error: e.to_string() })),
    }
}

static SOURCE_ARCHIVE: &'static [u8] = include_bytes!(env!("SOURCE_ARCHIVE"));
static INDEX_PAGE: &'static str = include_str!(env!("INDEX_PAGE"));

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
    async fn index(&self) -> Html<String> {
        Html(INDEX_PAGE.to_string())
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

    /// Add new item with some attrs
    ///
    /// TODO limit number of attrs? or just payload size?
    #[oai(path = "/add-item", method = "post")]
    async fn add_item(
            &self,
            payload: Json<ReqAddItem>,
            ) -> DBResponse<ResAddItem> {

        into_db_response(
            db::add_item(
                &self.state.pool,
                &payload
                ).await
            )
    }


    /// Add media item
    ///
    /// doesn't work yet
    #[oai(path = "/add-media", method = "post")]
    async fn add_media(
            &self,
            payload: Binary<Vec<u8>>,
            ) -> DBResponse<ResAddMedia> {

        //let mut reader = data.0.into_async_read();
        //let mut bytes = Vec::new();
        //reader.read_to_end(&mut bytes).await.map_err(BadRequest)?;
        //Ok(Json(bytes.len()))

        into_db_response(
            media::add_media(
                (&payload).to_vec()
                ).await
            )
    }
}


