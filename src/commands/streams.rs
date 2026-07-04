use crate::{
    StreamInfo,
    commands::{traits::MistCommand, utils::deserialize_streams_map},
    models::Stream,
};

use serde::{Deserialize, Serialize};
use serde_tuple::Deserialize_tuple;
use std::collections::HashMap;

/// Payload for the `addstream` command.
///
/// Sent to the Mist API to create or update one or more streams. The map keys are stream names,
/// and the values are their full [`Stream`] configurations. The serialized JSON places everything
/// under an `"addstream"` object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddStreamCommand {
    pub addstream: HashMap<String, Stream>,
}

impl AddStreamCommand {
    /// Creates a new command with the given stream map.
    pub fn new(streams: HashMap<String, Stream>) -> Self {
        Self { addstream: streams }
    }
}

impl From<HashMap<String, Stream>> for AddStreamCommand {
    fn from(value: HashMap<String, Stream>) -> Self {
        Self::new(value)
    }
}

impl From<Stream> for AddStreamCommand {
    fn from(value: Stream) -> Self {
        let mut hashmap = HashMap::new();
        hashmap.insert(value.name.clone(), value);
        Self::new(hashmap)
    }
}

impl MistCommand for AddStreamCommand {
    type Response = StreamCommandsResponse;
    const NAME: &'static str = "addstream";
}

/// Command to retrieve the list of all configured streams.
///
/// Sending an empty object (`{}`) to the `liststream` endpoint returns a full map of stream
/// names to their details. The response is captured by [`StreamCommandsResponse`].
#[derive(Serialize, Debug, Clone)]
pub struct StreamListCommand {
    pub streams: (),
}

impl MistCommand for StreamListCommand {
    type Response = StreamCommandsResponse;
    const NAME: &'static str = "liststream";
}

/// Response returned by stream‑listing or stream‑addition commands.
///
/// The Mist API wraps the stream information inside a `"streams"` key. This struct deserializes
/// that map, using a custom deserializer to handle the various possible field types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCommandsResponse {
    #[serde(deserialize_with = "deserialize_streams_map")]
    pub streams: HashMap<String, StreamInfo>,
}

/// Command to delete one or more streams by name.
///
/// The Mist API accepts the stream names as a JSON array under the `"deletestream"` field.
/// This command supports deleting multiple streams in a single request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStreamCommand {
    pub deletestream: Vec<String>,
}

impl DeleteStreamCommand {
    /// Creates a new deletion command for the given stream names.
    pub fn new(names: Vec<String>) -> Self {
        Self {
            deletestream: names,
        }
    }
}

impl MistCommand for DeleteStreamCommand {
    /// The Mist server sometimes returns no useful response; this type is kept as `Option`
    /// to gracefully handle missing or unexpected replies.
    type Response = Option<DeleteStreamResponse>;
    const NAME: &'static str = "deletestream";
}

/// Response payload for a stream deletion request.
///
/// Contains the updated stream information after deletion, wrapped in a `"streams"` map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStreamResponse {
    #[serde(deserialize_with = "deserialize_streams_map")]
    streams: HashMap<String, StreamInfo>,
}

/// Command to delete the source files of one or more streams.
///
/// This action removes the source files associated with the named streams, without affecting
/// other streams. The command accepts a list of stream names under the `"deletestreamsource"` key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStreamSourceCommand {
    deletestreamsource: Vec<String>,
}

impl DeleteStreamSourceCommand {
    /// Creates a new command with the given list of stream names.
    pub fn new(names: Vec<String>) -> Self {
        Self {
            deletestreamsource: names,
        }
    }
}

impl MistCommand for DeleteStreamSourceCommand {
    type Response = StreamCommandsResponse;
    const NAME: &'static str = "deletestreamsource";
}

/// Status of a delete‑source operation for a single stream.
///
/// The Mist API returns a string like `"0 No action"` or `"1 Source deleted"`.
/// This enum parses that string into a structured representation.
#[derive(Debug, Clone, PartialEq)]
pub enum DeleteStreamSourceStatus {
    /// No action was taken (e.g., stream did not exist).
    NoAction,
    /// The source file was successfully deleted.
    SourceDeleted,
    /// Both the source file and its DTSH index were deleted.
    SourceAndDtshDeleted,
    /// An unknown status code was received.
    Unknown(i32, String),
}

impl<'de> Deserialize<'de> for DeleteStreamSourceStatus {
    fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Some((code_str, msg)) = s.split_once(' ')
            && let Ok(code) = code_str.parse::<i32>()
        {
            return Ok(match code {
                0 => DeleteStreamSourceStatus::NoAction,
                1 => DeleteStreamSourceStatus::SourceDeleted,
                2 => DeleteStreamSourceStatus::SourceAndDtshDeleted,
                _ => DeleteStreamSourceStatus::Unknown(code, msg.to_string()),
            });
        }
        Ok(DeleteStreamSourceStatus::Unknown(0, s))
    }
}

/// Response from a `deletestreamsource` command.
///
/// The Mist API may return a single status, an array of statuses, or a map of stream names to
/// statuses. This enum covers all three forms.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum DeleteStreamSourceReponse {
    /// A single status (when only one stream was requested).
    Single(DeleteStreamSourceStatus),
    /// An array of statuses, in the same order as the requested stream names.
    Array(Vec<DeleteStreamSourceStatus>),
    /// A map from stream name to its individual status.
    Object(HashMap<String, DeleteStreamSourceStatus>),
}

/// Command to completely remove a stream and all its associated data.
///
/// This is a destructive operation that nukes the stream entirely. It expects a single
/// stream name under the `"nuke_stream"` key.
#[derive(Debug, Clone, Serialize)]
pub struct NukeStreamCommand {
    nuke_stream: String,
}

impl NukeStreamCommand {
    /// Creates a new nuke command for the given stream name.
    pub fn new(name: String) -> Self {
        Self { nuke_stream: name }
    }
}

impl MistCommand for NukeStreamCommand {
    type Response = ();
    const NAME: &'static str = "nuke_stream";
}

/// Command to retrieve real‑time statistics for all active streams.
///
/// The `"active_streams"` endpoint returns a wealth of metrics for each currently active stream.
/// This command includes a fixed list of requested fields; the response is captured in
/// [`ListActiveStreamsResponse`].
#[derive(Debug, Clone, Serialize)]
pub struct ListActiveStreamsCommand {
    active_streams: Vec<&'static str>,
}

impl ListActiveStreamsCommand {
    /// Creates a new command with the default set of statistical fields.
    pub fn new() -> Self {
        Self {
            active_streams: vec![
                "clients",
                "lastms",
                "firstms",
                "viewers",
                "inputs",
                "outputs",
                "views",
                "viewseconds",
                "upbytes",
                "downbytes",
                "packsent",
                "packloss",
                "packretrans",
                "status",
                "health",
            ],
        }
    }
}

impl MistCommand for ListActiveStreamsCommand {
    type Response = ListActiveStreamsResponse;
    const NAME: &'static str = "active_streams";
}

/// Response for the `active_streams` command.
///
/// Contains an optional map from stream names to their detailed statistical information.
#[derive(Debug, Clone, Deserialize)]
pub struct ListActiveStreamsResponse {
    active_streams: Option<HashMap<String, ActiveStreamStats>>,
}

/// Statistical information for a single active stream.
///
/// These fields correspond to the metrics requested by [`ListActiveStreamsCommand`].
#[derive(Debug, Clone, Deserialize_tuple)]
struct ActiveStreamStats {
    clients: Option<i32>,
    lastms: Option<i32>,
    firstms: Option<i32>,
    viewers: Option<i32>,
    inputs: Option<i32>,
    outputs: Option<i32>,
    views: Option<i32>,
    viewseconds: Option<i32>,
    upbytes: Option<i32>,
    downbytes: Option<i32>,
    packsent: Option<i32>,
    packloss: Option<i32>,
    packretrans: Option<i32>,
    status: Option<String>,
    health: Option<ActiveStreamHealth>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackInfo {
    pub bframes: bool,
    pub buffer: i32,
    pub codec: String,
    pub efpks: i32,
    pub efps: i32,
    pub fpks: i32,
    pub fps: i32,
    pub height: i32,
    pub id: i32,
    pub idx: i32,
    pub jitter: i32,
    pub kbits: i32,
    pub keys: KeyInfo,
    pub width: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KeyInfo {
    pub frame_ms_max: i32,
    pub frame_ms_min: i32,
    pub frames_max: i32,
    pub frames_min: i32,
    pub ms_max: i32,
    pub ms_min: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActiveStreamHealth {
    pub buffer: i32,
    pub issues: String,
    pub jitter: i32,
    pub tracks: Vec<String>,
    #[serde(flatten)]
    pub tracks_detail: HashMap<String, TrackInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{commands::traits::MistCommand, models::Stream};
    use serde_json::{Value, json};
    use std::collections::HashMap;

    fn sample_stream() -> Stream {
        Stream {
            source: "push://".to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn stream_add_command_new() {
        let mut streams = HashMap::new();
        streams.insert("camera1".to_string(), sample_stream());

        let command = AddStreamCommand::new(streams.clone());

        assert_eq!(command.addstream.len(), 1);
    }

    #[test]
    fn stream_add_command_serialization() {
        let mut streams = HashMap::new();
        streams.insert("camera1".to_string(), sample_stream());

        let command = AddStreamCommand::new(streams);

        let value = serde_json::to_value(command).unwrap();

        assert!(value.get("addstream").is_some());
        assert!(value["addstream"].get("camera1").is_some());
    }

    #[test]
    fn stream_add_command_name() {
        assert_eq!(AddStreamCommand::NAME, "addstream");
    }

    #[test]
    fn delete_stream_command_new() {
        let names = vec![
            "stream1".to_string(),
            "stream2".to_string(),
            "stream3".to_string(),
        ];

        let command = DeleteStreamCommand::new(names.clone());

        assert_eq!(command.deletestream, names);
    }

    #[test]
    fn delete_stream_command_serialization() {
        let command = DeleteStreamCommand::new(vec!["stream1".to_string(), "stream2".to_string()]);

        let expected = json!({
            "deletestream": [
                "stream1",
                "stream2"
            ]
        });

        let actual = serde_json::to_value(command).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn delete_stream_command_name() {
        assert_eq!(DeleteStreamCommand::NAME, "deletestream");
    }

    #[test]
    fn delete_stream_response_deserializes_none() {
        let value = Value::Null;

        let response: Option<Value> = serde_json::from_value(value).unwrap();

        assert!(response.is_none());
    }

    #[test]
    fn delete_stream_response_deserializes_some() {
        let value = json!({
            "status": "OK"
        });

        let response: Option<Value> = serde_json::from_value(value.clone()).unwrap();

        assert_eq!(response.unwrap(), value);
    }

    #[test]
    fn stream_add_response_deserializes() {
        let json = json!({
            "streams": {
                "camera1": {
                    "online": 1,
                    "source": "push://",
                    "name": "camera1"
                }
            }
        });

        let response: StreamCommandsResponse = serde_json::from_value(json).unwrap();

        assert!(response.streams.contains_key("camera1"));
    }

    #[test]
    fn stream_add_response_empty_streams() {
        let json = json!({
            "streams": {}
        });

        let response: StreamCommandsResponse = serde_json::from_value(json).unwrap();

        assert!(response.streams.is_empty());
    }
}
