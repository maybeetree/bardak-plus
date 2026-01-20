use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub rows: Vec<ResponseGetLatestRowsInner>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseGetLatestRowsInner {
    pub item_id: i64,
    pub attr_name: String,
    pub attr_val: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetLatestItems {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default = "default_offset")]
    pub offset: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseGetLatestItems {
    pub items: Vec<ResponseGetLatestItemsInner>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseGetLatestItemsInner {
    pub item_id: i64,
    pub attrs: HashMap<String, String>,
}

fn default_limit() -> i64 {
    100
}

fn default_offset() -> i64 {
    0
}

