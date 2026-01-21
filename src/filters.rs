use crate::handlers;
use crate::app::App;
use crate::schema;
use warp::Filter;
use std::sync::Arc;

/// This is the main filter. It puts all other filters together,
/// like a wall of swiss cheese.
pub fn root(
    app: Arc<App>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    (
        hello(app.clone())
    ).or(
        latest_rows(app.clone())
    ).or(
        latest_items(app.clone())
    )
}

pub fn hello(
    _app: Arc<App>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and_then(handlers::hello)
}

pub fn latest_rows(
    app: Arc<App>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("latest-rows")
        .and(warp::get())
        .and(with_app(app))
        .and(warp::query::<schema::GetLatestRows>())
        .and_then(handlers::latest_rows)
}

pub fn latest_items(
    app: Arc<App>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("latest-items")
        .and(warp::get())
        .and(with_app(app))
        .and(warp::query::<schema::GetLatestItems>())
        .and_then(handlers::latest_items)
}

fn with_app(app: Arc<App>) -> impl Filter<Extract = (Arc<App>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || app.clone())
}

