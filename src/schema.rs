use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use poem_openapi::payload::Json;
use poem_openapi::Object;
use poem_openapi::ApiResponse;
use poem_openapi::types::Type;
use poem_openapi::types::ToJSON;

// We use i64 for everything because that's
// the native datatype of sqlite.
// Using the more common i32 would involve casts everywhere.

/// Database action response
#[derive(ApiResponse)]
pub enum DBResponse<T: Type + ToJSON> {
    /// Success.
    #[oai(status = 200)]
    Ok(Json<T>),

    /// Any error.
    #[oai(status = 500)]
    Error(Json<DBError>),
}

/// Database error
#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct DBError {
    pub error: String,
}


///// Get latest rows request schema
//#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
//pub struct ReqGetLatestRows {
//    #[serde(default = "default_limit")]
//    pub limit: i64,
//    #[serde(default = "default_offset")]
//    pub offset: i64,
//}

/// Get latest rows response schema
#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResGetLatestRows {
    pub rows: Vec<ResGetLatestRowsInner>,
}


/// Get latest rows response schema (single item)
#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResGetLatestRowsInner {
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

#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResponseGetLatestItems {
    pub items: Vec<ResponseGetLatestItemsInner>,
}

#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResponseGetLatestItemsInner {
    pub item_id: i64,
    pub attrs: HashMap<String, String>,
}

pub fn default_limit() -> i64 {
    100
}

pub fn default_offset() -> i64 {
    0
}

