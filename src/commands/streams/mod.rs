mod active_streams;
mod add_stream;
mod delete_stream;
mod list_stream;
mod nuke_stream;
mod stream_tags;
mod tag_stream;

pub use active_streams::*;
pub use add_stream::*;
pub use delete_stream::*;
pub use list_stream::*;
pub use nuke_stream::*;
pub use stream_tags::*;
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
