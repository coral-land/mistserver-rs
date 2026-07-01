use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ProtocolOnlineValue {
    String(String),
    Number(i8),
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Protocol {
    connector: ProtocolConnector,
    online: ProtocolOnlineValue,
}
