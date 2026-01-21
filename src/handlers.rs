
//pub async fn list_todos(app: ListOptions, db: Db) -> Result<impl warp::Reply, Infallible> {

pub async fn get_latest_rows(
        self: Arc<Self>,
        payload: schema::GetLatestRows,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {

    // Here the response type is Box because
    // there are two possible responses
    // warp::reply::json and String
    // which have different sizes
    // so we need to have indirection.

    Ok(match db::get_latest_rows(&self.pool, &payload).await {
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

