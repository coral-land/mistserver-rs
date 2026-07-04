use crate::commands::shared::deserialize_streams_map;
use crate::{
    StreamInfo,
    commands::{streams::StreamCommandsResponse, traits::MistCommand},
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Command for deleting one or more streams from the Mist server.
///
/// The command accepts one or more stream names and submits them to the
/// `deletestream` API endpoint. Multiple streams can be removed in a single
/// request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStreamCommand {
    /// Names of the streams to delete.
    pub deletestream: Vec<String>,
}

impl DeleteStreamCommand {
    /// Creates a new delete-stream command.
    pub fn new(names: Vec<String>) -> Self {
        Self {
            deletestream: names,
        }
    }
}

impl MistCommand for DeleteStreamCommand {
    /// Mist may return no payload after a successful deletion, so the response
    /// is represented as an optional value.
    type Response = Option<DeleteStreamResponse>;

    /// Mist API command name.
    const NAME: &'static str = "deletestream";
}

/// Response returned after deleting one or more streams.
///
/// When present, the response contains the remaining stream definitions
/// reported by the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStreamResponse {
    #[serde(deserialize_with = "deserialize_streams_map")]
    streams: HashMap<String, StreamInfo>,
}

/// Command for deleting the backing source files of one or more streams.
///
/// Unlike [`DeleteStreamCommand`], this command removes the media source files
/// associated with the specified streams.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStreamSourceCommand {
    /// Names of the streams whose source files should be removed.
    deletestreamsource: Vec<String>,
}

impl DeleteStreamSourceCommand {
    /// Creates a new delete-stream-source command.
    pub fn new(names: Vec<String>) -> Self {
        Self {
            deletestreamsource: names,
        }
    }
}

impl MistCommand for DeleteStreamSourceCommand {
    type Response = StreamCommandsResponse;

    /// Mist API command name.
    const NAME: &'static str = "deletestreamsource";
}

/// Result of deleting the source file for a single stream.
///
/// Mist encodes these values as strings containing both a numeric status code
/// and a human-readable message (for example `"1 Source deleted"`). This type
/// converts those values into a structured Rust representation.
#[derive(Debug, Clone, PartialEq)]
pub enum DeleteStreamSourceStatus {
    /// No action was performed.
    NoAction,

    /// The stream's source file was removed.
    SourceDeleted,

    /// Both the source file and its generated DTSH index were removed.
    SourceAndDtshDeleted,

    /// An unrecognized status code returned by the server.
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

/// Response returned by the `deletestreamsource` endpoint.
///
/// Depending on the number of requested streams and the server version, Mist
/// may return:
///
/// - a single status,
/// - an array of statuses,
/// - or a map from stream name to status.
///
/// This enum transparently supports all supported response formats.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum DeleteStreamSourceReponse {
    /// Status for a single stream.
    Single(DeleteStreamSourceStatus),

    /// Statuses returned in request order.
    Array(Vec<DeleteStreamSourceStatus>),

    /// Statuses keyed by stream name.
    Object(HashMap<String, DeleteStreamSourceStatus>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::traits::MistCommand;
    use serde_json::json;

    #[test]
    fn delete_stream_command_new() {
        let names = vec!["cam1".to_string(), "cam2".to_string()];

        let command = DeleteStreamCommand::new(names.clone());

        assert_eq!(command.deletestream, names);
    }

    #[test]
    fn delete_stream_command_serialization() {
        let command = DeleteStreamCommand::new(vec!["cam1".to_string(), "cam2".to_string()]);

        let expected = json!({
            "deletestream": [
                "cam1",
                "cam2"
            ]
        });

        assert_eq!(serde_json::to_value(command).unwrap(), expected);
    }

    #[test]
    fn delete_stream_command_name() {
        assert_eq!(DeleteStreamCommand::NAME, "deletestream");
    }

    #[test]
    fn delete_stream_response_option_none() {
        let value = serde_json::Value::Null;

        let response: Option<DeleteStreamResponse> = serde_json::from_value(value).unwrap();

        assert!(response.is_none());
    }

    #[test]
    fn delete_stream_source_command_new() {
        let names = vec!["cam1".to_string()];

        let command = DeleteStreamSourceCommand::new(names.clone());

        let value = serde_json::to_value(command).unwrap();

        assert_eq!(value["deletestreamsource"], json!(names));
    }

    #[test]
    fn delete_stream_source_command_name() {
        assert_eq!(DeleteStreamSourceCommand::NAME, "deletestreamsource");
    }

    #[test]
    fn deserialize_no_action_status() {
        let value = json!("0 No action");

        let status: DeleteStreamSourceStatus = serde_json::from_value(value).unwrap();

        assert_eq!(status, DeleteStreamSourceStatus::NoAction);
    }

    #[test]
    fn deserialize_source_deleted_status() {
        let value = json!("1 Source deleted");

        let status: DeleteStreamSourceStatus = serde_json::from_value(value).unwrap();

        assert_eq!(status, DeleteStreamSourceStatus::SourceDeleted);
    }

    #[test]
    fn deserialize_source_and_dtsh_deleted_status() {
        let value = json!("2 Source and DTSH deleted");

        let status: DeleteStreamSourceStatus = serde_json::from_value(value).unwrap();

        assert_eq!(status, DeleteStreamSourceStatus::SourceAndDtshDeleted);
    }

    #[test]
    fn deserialize_unknown_numeric_status() {
        let value = json!("99 Something unexpected");

        let status: DeleteStreamSourceStatus = serde_json::from_value(value).unwrap();

        assert_eq!(
            status,
            DeleteStreamSourceStatus::Unknown(99, "Something unexpected".to_string())
        );
    }

    #[test]
    fn deserialize_unknown_non_numeric_status() {
        let value = json!("unexpected");

        let status: DeleteStreamSourceStatus = serde_json::from_value(value).unwrap();

        assert_eq!(
            status,
            DeleteStreamSourceStatus::Unknown(0, "unexpected".to_string())
        );
    }

    #[test]
    fn delete_stream_source_response_single() {
        let json = json!("1 Source deleted");

        let response: DeleteStreamSourceReponse = serde_json::from_value(json).unwrap();

        match response {
            DeleteStreamSourceReponse::Single(status) => {
                assert_eq!(status, DeleteStreamSourceStatus::SourceDeleted);
            }
            _ => panic!("expected single response"),
        }
    }

    #[test]
    fn delete_stream_source_response_array() {
        let json = json!(["0 No action", "1 Source deleted"]);

        let response: DeleteStreamSourceReponse = serde_json::from_value(json).unwrap();

        match response {
            DeleteStreamSourceReponse::Array(values) => {
                assert_eq!(values.len(), 2);
                assert_eq!(values[0], DeleteStreamSourceStatus::NoAction);
                assert_eq!(values[1], DeleteStreamSourceStatus::SourceDeleted);
            }
            _ => panic!("expected array response"),
        }
    }

    #[test]
    fn delete_stream_source_response_object() {
        let json = json!({
            "cam1": "1 Source deleted",
            "cam2": "0 No action"
        });

        let response: DeleteStreamSourceReponse = serde_json::from_value(json).unwrap();

        match response {
            DeleteStreamSourceReponse::Object(map) => {
                assert_eq!(map["cam1"], DeleteStreamSourceStatus::SourceDeleted);
                assert_eq!(map["cam2"], DeleteStreamSourceStatus::NoAction);
            }
            _ => panic!("expected object response"),
        }
    }
}
