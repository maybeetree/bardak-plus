use crate::app::App;
use crate::schema;
use std::convert::Infallible;
use std::sync::Arc;
use crate::db;
use warp::reply::with_status;
use warp::http::StatusCode;

pub async fn hello() -> Result<String, warp::Rejection> {
    Ok(
        format!(
            "Hello! I am {} version {}. \
            I am licensed under {}, \
            and my source code is at {}.",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_LICENSE"),
            env!("CARGO_PKG_REPOSITORY"),
        ).to_string()
    )
}

//pub async fn list_todos(app: ListOptions, db: Db) -> Result<impl warp::Reply, Infallible> {

pub async fn latest_rows(
        app: Arc<App>,
        payload: schema::GetLatestRows,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {

    // Here the response type is Box because
    // there are two possible responses
    // warp::reply::json and String
    // which have different sizes
    // so we need to have indirection.

    Ok(match db::latest_rows(&app.pool, &payload).await {
        Ok(v) => Box::new(with_status(
            warp::reply::json(&v),
            StatusCode::OK
            )),
        Err(e) => Box::new(with_status(
            format!("{:#?}", e),
            StatusCode::INTERNAL_SERVER_ERROR
            )),
    })
}

pub async fn latest_items(
        app: Arc<App>,
        payload: schema::GetLatestItems,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {

    // TODO copypasta

    Ok(match db::latest_items(&app.pool, &payload).await {
        Ok(v) => Box::new(with_status(
            warp::reply::json(&v),
            StatusCode::OK
            )),
        Err(e) => Box::new(with_status(
            format!("{:#?}", e),
            StatusCode::INTERNAL_SERVER_ERROR
            )),
    })
}
