use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
    source: String,
    always_on: Option<bool>,
    buffer_time: Option<u64>,
    debug: Option<u64>,
    fallback_stream: Option<String>,
}
