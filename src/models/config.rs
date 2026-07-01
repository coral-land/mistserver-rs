use crate::Protocol;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MistConfig {
    /// Mistserver server identifier
    serverid: String,

    /// The accesslog path for mistserver if defined
    accesslog: Option<String>,

    /// Global debug level for mistserver if defined
    debug: Option<i32>,

    /// Session input mode for mistserver
    #[serde(rename = "sessionInputMode")]
    session_input_mode: Option<i32>,

    /// Session output mode for mistserver
    #[serde(rename = "sessionOutputMode")]
    session_output_mode: i32,

    /// Session stream info mode for mistserver
    #[serde(rename = "sessionStreamInfoMode")]
    session_stream_info_mode: String,

    /// Session unspecified mode for mistserver
    #[serde(rename = "sessionUnspecifiedMode")]
    session_unspecified_mode: i32,

    /// Default stream name for mistserver if configured
    #[serde(rename = "defaultStream")]
    default_stream: Option<String>,

    /// Session viewer mode for mistserver
    #[serde(rename = "sessionViewerMode")]
    session_viewer_mode: i32,

    /// Time interval or tick value used by mistserver
    time: i32,

    /// Token mode setting for mistserver
    tkn_mode: i32,

    /// Mistserver version identifier
    version: String,

    /// Host name or address for mistserver
    host: String,

    /// Mistserver instance id
    iid: String,

    /// Mistserver version string, often same as version
    mistver: String,

    /// Supported transport protocols in mistserver configuration
    protocols: Vec<Protocol>,
}
