use crate::commands::{streams::StreamCommandsResponse, traits::MistCommand};
use serde::Serialize;

/// Command for retrieving all configured streams from the Mist server.
///
/// The `liststream` endpoint expects an empty object (`{}`) as its payload and
/// responds with the complete collection of configured streams.
#[derive(Debug, Clone, Serialize)]
pub struct StreamListCommand {
    /// Empty payload required by the Mist API.
    pub streams: (),
}

impl MistCommand for StreamListCommand {
    type Response = StreamCommandsResponse;

    /// Mist API command name.
    const NAME: &'static str = "liststream";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::traits::MistCommand;
    use serde_json::json;

    #[test]
    fn stream_list_command_serialization() {
        let command = StreamListCommand { streams: () };

        let expected = json!({
            "streams": null
        });

        let actual = serde_json::to_value(command).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn stream_list_command_name() {
        assert_eq!(StreamListCommand::NAME, "liststream");
    }

    #[test]
    fn stream_list_command_roundtrip_serialization() {
        let command = StreamListCommand { streams: () };

        let value = serde_json::to_value(&command).unwrap();

        assert!(value.is_object());
        assert!(value.get("streams").is_some());
        assert!(value["streams"].is_null());
    }
}
