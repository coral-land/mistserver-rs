use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    Stream,
    commands::{streams::StreamCommandsResponse, traits::MistCommand},
};

/// Command for creating or updating streams in the Mist API.
///
/// The payload consists of a map where each key is a stream name and each
/// value is the corresponding [`Stream`] configuration. Mist allows multiple
/// streams to be submitted in a single request, making this command suitable
/// for both bulk creation and updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddStreamCommand {
    /// Collection of streams to create or update, keyed by stream name.
    pub addstream: HashMap<String, Stream>,
}

impl AddStreamCommand {
    /// Creates a new command from a collection of stream definitions.
    pub fn new(streams: HashMap<String, Stream>) -> Self {
        Self { addstream: streams }
    }
}

/// Converts a map of streams into an [`AddStreamCommand`].
///
/// This allows a `HashMap<String, Stream>` to be passed directly wherever an
/// `AddStreamCommand` is expected.
impl From<HashMap<String, Stream>> for AddStreamCommand {
    fn from(value: HashMap<String, Stream>) -> Self {
        Self::new(value)
    }
}

/// Converts a single [`Stream`] into an [`AddStreamCommand`].
///
/// The stream is inserted into a new map using its `name` field as the key,
/// allowing a single stream to be submitted without manually constructing a
/// `HashMap`.
impl From<Stream> for AddStreamCommand {
    fn from(value: Stream) -> Self {
        let mut hashmap = HashMap::new();
        hashmap.insert(value.name.clone(), value);
        Self::new(hashmap)
    }
}

impl MistCommand for AddStreamCommand {
    type Response = StreamCommandsResponse;

    /// Mist API command name.
    const NAME: &'static str = "addstream";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{commands::traits::MistCommand, models::Stream};
    use serde_json::json;
    use std::collections::HashMap;

    fn sample_stream(name: &str) -> Stream {
        Stream {
            name: name.to_string(),
            source: "push://".to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn add_stream_command_new() {
        let mut streams = HashMap::new();
        streams.insert("camera1".to_string(), sample_stream("camera1"));

        let command = AddStreamCommand::new(streams.clone());

        assert_eq!(command.addstream.len(), 1);
        assert_eq!(command.addstream["camera1"].source, "push://");
    }

    #[test]
    fn add_stream_command_new_empty() {
        let command = AddStreamCommand::new(HashMap::new());

        assert!(command.addstream.is_empty());
    }

    #[test]
    fn add_stream_command_serialization() {
        let mut streams = HashMap::new();
        streams.insert("camera1".to_string(), sample_stream("camera1"));

        let command = AddStreamCommand::new(streams);

        let value = serde_json::to_value(command).unwrap();

        assert!(value.get("addstream").is_some());
        assert!(value["addstream"].get("camera1").is_some());
        assert_eq!(value["addstream"]["camera1"]["source"], json!("push://"));
    }

    #[test]
    fn add_stream_command_deserialization() {
        let json = json!({
            "addstream": {
                "camera1": {
                    "name": "camera1",
                    "source": "push://"
                }
            }
        });

        let command: AddStreamCommand = serde_json::from_value(json).unwrap();

        assert_eq!(command.addstream.len(), 1);
        assert!(command.addstream.contains_key("camera1"));
        assert_eq!(command.addstream["camera1"].source, "push://");
    }

    #[test]
    fn add_stream_command_from_hashmap() {
        let mut streams = HashMap::new();
        streams.insert("camera1".to_string(), sample_stream("camera1"));

        let command: AddStreamCommand = streams.clone().into();

        assert_eq!(command.addstream.len(), streams.len());
        assert!(command.addstream.contains_key("camera1"));
    }

    #[test]
    fn add_stream_command_from_stream() {
        let stream = sample_stream("camera1");

        let command: AddStreamCommand = stream.into();

        assert_eq!(command.addstream.len(), 1);
        assert!(command.addstream.contains_key("camera1"));
        assert_eq!(command.addstream["camera1"].source, "push://");
    }

    #[test]
    fn add_stream_command_name() {
        assert_eq!(AddStreamCommand::NAME, "addstream");
    }

    #[test]
    fn add_stream_command_roundtrip() {
        let mut streams = HashMap::new();
        streams.insert("camera1".to_string(), sample_stream("camera1"));
        streams.insert("camera2".to_string(), sample_stream("camera2"));

        let command = AddStreamCommand::new(streams);

        let json = serde_json::to_value(&command).unwrap();

        let decoded: AddStreamCommand = serde_json::from_value(json).unwrap();

        assert_eq!(decoded.addstream.len(), 2);
        assert!(decoded.addstream.contains_key("camera1"));
        assert!(decoded.addstream.contains_key("camera2"));
    }
}
