use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Protocol {
    /// The most important part. Actual type of protocol that is defined as
    /// `connector`.
    /// The type of this field is an enum called `ProtocolConnector`.
    pub connector: ProtocolConnector,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<ProtocolStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctrlprefix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_track_sorting: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceservers: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub extra_fields: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolStatus {
    String(String),
    Number(i8),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ProtocolConnector {
    AAC,
    CMAF,
    DTSC,
    EBML,
    FLAC,
    FLV,
    H264,
    HDS,
    HLS,
    HTTP,
    HTTPS,
    JPG,
    JSON,
    MP3,
    MP4,
    OGG,
    RTMP,
    SDP,
    SubRip,
    TSSRT,
    WAV,
    WebRTC,
}
