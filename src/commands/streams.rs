use crate::{
    StreamInfo,
    commands::{traits::MistCommand, utils::deserialize_streams_map},
    models::Stream,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Command payload for adding one or more streams.
///
/// This struct is serialized into JSON and sent as the `addstream` command
/// to the Mist API. The keys are stream names and the values are stream
/// configurations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamAddCommand {
    pub addstream: HashMap<String, Stream>,
}

impl StreamAddCommand {
    /// Creates a new `StreamAddCommand` with the given stream configurations.
    ///
    /// # Arguments
    /// * `streams` - A map from stream names to their `Stream` configurations.
    pub fn new(streams: HashMap<String, Stream>) -> Self {
        Self { addstream: streams }
    }
}

impl From<HashMap<String, Stream>> for StreamAddCommand {
    fn from(value: HashMap<String, Stream>) -> Self {
        Self::new(value)
    }
}

impl From<Stream> for StreamAddCommand {
    fn from(value: Stream) -> Self {
        let mut hashmap = HashMap::new();
        hashmap.insert(value.name.clone(), value);

        Self::new(hashmap)
    }
}

/// Implementation of MistCommand
impl MistCommand for StreamAddCommand {
    type Response = StreamAddCommandResponse;
    const NAME: &'static str = "addstream";
}

/// Response received after successfully adding streams.
///
/// Contains a map of stream names to their detailed information as returned
/// by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamAddCommandResponse {
    /// Map of stream names to their `StreamInfo` details.
    #[serde(deserialize_with = "deserialize_streams_map")]
    pub streams: HashMap<String, StreamInfo>,
}

/// Command for deleting one or more streams.
///
/// The Mist API accepts multiple formats for deletion:
/// - A single stream name as a string.
/// - An array of stream names.
/// - A more complex object (hash map) for advanced deletion criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStreamCommand {
    // Stream name list
    pub deletestream: Vec<String>,
}

impl DeleteStreamCommand {
    /// Creates a new `DeleteStreamCommand` with the given stream names.
    ///
    /// # Arguments
    /// * `names` - A vector of stream names to be deleted.
    pub fn new(names: Vec<String>) -> Self {
        Self {
            deletestream: names,
        }
    }
}

impl MistCommand for DeleteStreamCommand {
    /// There is no valid response from mist server for this command
    /// It returns whole cluster information witch we do not need.
    /// So we make it optional to avoid problems in desrialization.
    type Response = Option<Value>;
    const NAME: &'static str = "deletestream";
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

        let command = StreamAddCommand::new(streams.clone());

        assert_eq!(command.addstream.len(), 1);
    }

    #[test]
    fn stream_add_command_serialization() {
        let mut streams = HashMap::new();
        streams.insert("camera1".to_string(), sample_stream());

        let command = StreamAddCommand::new(streams);

        let value = serde_json::to_value(command).unwrap();

        assert!(value.get("addstream").is_some());
        assert!(value["addstream"].get("camera1").is_some());
    }

    #[test]
    fn stream_add_command_name() {
        assert_eq!(StreamAddCommand::NAME, "addstream");
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

        let response: StreamAddCommandResponse = serde_json::from_value(json).unwrap();

        assert!(response.streams.contains_key("camera1"));
    }

    #[test]
    fn stream_add_response_empty_streams() {
        let json = json!({
            "streams": {}
        });

        let response: StreamAddCommandResponse = serde_json::from_value(json).unwrap();

        assert!(response.streams.is_empty());
    }
}
