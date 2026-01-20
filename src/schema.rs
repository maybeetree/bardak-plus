use serde::{Deserialize, Serialize};

// We use i64 for everything because that's
// the native datatype of sqlite.
// Using the more common i32 would involve casts everywhere.

#[derive(Debug, Deserialize, Serialize)]
pub struct GetLatestRows {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default = "default_offset")]
    pub offset: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseGetLatestRows {
    pub top: Vec<ResponseGetLatestRowsInner>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseGetLatestRowsInner {
    pub item_id: i64,
    pub attr_name: String,
    pub attr_val: String,
}

fn default_limit() -> i64 {
    4
}

fn default_offset() -> i64 {
    0
}

