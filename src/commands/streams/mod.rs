mod active_streams;
mod addstream;
mod deletestream;
mod liststream;
mod nuke_stream;
mod tag_stream;

pub use active_streams::*;
pub use addstream::*;
pub use deletestream::*;
pub use liststream::*;
pub use nuke_stream::*;
pub use tag_stream::*;

use crate::{StreamInfo, commands::shared::deserialize_streams_map};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response returned by stream‑listing or stream‑addition commands.
///
/// The Mist API wraps the stream information inside a `"streams"` key. This struct deserializes
/// that map, using a custom deserializer to handle the various possible field types.
// TODO: Move this to shared or something.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCommandsResponse {
    #[serde(deserialize_with = "deserialize_streams_map")]
    pub streams: HashMap<String, StreamInfo>,
}
