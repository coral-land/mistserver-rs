use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
    pub source: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_on: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_time: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_stream: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    pub name: String,
    pub source: String,
    pub error: String,
    pub online: i32,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
