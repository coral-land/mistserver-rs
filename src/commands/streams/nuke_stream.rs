use crate::commands::traits::MistCommand;
use serde::Serialize;

/// Command for permanently removing a stream and all associated data.
///
/// Unlike a regular stream deletion, this command performs a destructive
/// operation that removes the stream and any related resources managed by the
/// Mist server.
#[derive(Debug, Clone, Serialize)]
pub struct NukeStreamCommand {
    /// Name of the stream to permanently remove.
    nuke_stream: String,
}

impl NukeStreamCommand {
    /// Creates a new command targeting the specified stream.
    pub fn new(name: String) -> Self {
        Self { nuke_stream: name }
    }
}

impl MistCommand for NukeStreamCommand {
    type Response = Option<serde_json::Value>;

    /// Mist API command name.
    const NAME: &'static str = "nuke_stream";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::traits::MistCommand;
    use serde_json::json;

    #[test]
    fn nuke_stream_command_new() {
        let command = NukeStreamCommand::new("camera1".to_string());
        let value = serde_json::to_value(&command).unwrap();

        assert_eq!(value["nuke_stream"], "camera1");
    }

    #[test]
    fn nuke_stream_command_serialization() {
        let command = NukeStreamCommand::new("stream-a".to_string());
        let expected = json!({
            "nuke_stream": "stream-a"
        });

        let actual = serde_json::to_value(command).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn nuke_stream_command_name() {
        assert_eq!(NukeStreamCommand::NAME, "nuke_stream");
    }

    #[test]
    fn nuke_stream_command_accepts_empty_name() {
        let command = NukeStreamCommand::new(String::new());

        let value = serde_json::to_value(command).unwrap();

        assert_eq!(value["nuke_stream"], "");
    }

    #[test]
    fn nuke_stream_command_roundtrip_serialization() {
        let command = NukeStreamCommand::new("camera42".to_string());

        let value = serde_json::to_value(&command).unwrap();

        assert_eq!(value["nuke_stream"], "camera42");
        assert_eq!(value.as_object().unwrap().len(), 1);
    }
}
