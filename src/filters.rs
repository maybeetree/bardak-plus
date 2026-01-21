use crate::handlers;

pub fn latest_rows(
    app: App,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("latest-rows")
        .and(warp::get())
        .and(with_app(app))
        .and(warp::query::<schema::GetLatestRows>())
        .and_then(handlers::latest_rows)
}

fn with_app(app: Arc<App>) -> impl Filter<Extract = (App,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || app.clone())
}

