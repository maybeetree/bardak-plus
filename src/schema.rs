use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use poem_openapi::payload::Json;
use poem_openapi::payload::Attachment;
use poem_openapi::Object;
use poem_openapi::ApiResponse;
use poem_openapi::types::Type;
use poem_openapi::types::ToJSON;

// We use i64 for everything because that's
// the native datatype of sqlite.
// Using the more common i32 would involve casts everywhere.

#[derive(ApiResponse)]
pub enum BinResponse {
    /// Success.
    #[oai(status = 200)]
    Ok(Attachment<&'static [u8]>),

    /// Any error.
    #[oai(status = 500)]
    Error(Json<Error>),
}

/// Database action response
#[derive(ApiResponse)]
pub enum JsonResponse<T: Type + ToJSON> {
    /// Success.
    #[oai(status = 200)]
    Ok(Json<T>),

    /// Any error.
    #[oai(status = 500)]
    Error(Json<Error>),
}

/// Database error
#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub error: String,
}

/// Get latest rows response schema
#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResLatestRows {
    pub rows: Vec<ResLatestRowsInner>,
}


/// Get latest rows response schema (single item)
#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResLatestRowsInner {
    pub item_id: i64,
    pub attr_name: String,
    pub attr_val: String,
}

/// Get latest items response schema
#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResLatestItems {
    pub items: Vec<ResLatestItemsInner>,
}

/// Get latest items response schema (single item)
#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResLatestItemsInner {
    pub item_id: i64,
    pub attrs: HashMap<String, String>,
}

/// Request: create a new item with some attrs
#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReqAddItem {
    pub attrs: HashMap<String, String>,
}

/// Response: create a new item with some attrs
#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResAddItem {
    /// ID of the newly created item
    pub item_id: i64,
}

#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResAddMedia {
    /// ID of the newly created item
    pub task_id: String,
    pub media_id: String,
}

/// Get thumbs response schema
#[derive(Debug, Object, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResGetThumbNames {
    pub thumbs: Vec<String>,
}

pub fn default_limit() -> i64 {
    100
}

pub fn default_offset() -> i64 {
    0
}

pub fn default_spec() -> Option<String> {
    None
}


