use serde::{Deserialize, Serialize};

// We use i64 for everything because that's
// the native datatype of sqlite.
// Using the more common i32 would involve casts everywhere.

#[derive(Debug, Deserialize, Serialize)]
pub struct GetLatest {
    pub page: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseGetLatest {
    pub top: Vec<ResponseGetLatestInner>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseGetLatestInner {
    pub id: i64,
}

