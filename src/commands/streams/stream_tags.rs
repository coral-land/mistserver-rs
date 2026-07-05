use crate::commands::traits::MistCommand;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamTagsCommand {
    stream_tags: Vec<String>,
}

impl StreamTagsCommand {
    #[must_use]
    pub fn new(names: Vec<String>) -> Self {
        Self { stream_tags: names }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamTagsCommandResponse {
    stream_tags: HashMap<String, Vec<String>>,
}

impl MistCommand for StreamTagsCommand {
    type Response = StreamTagsCommandResponse;
    const NAME: &'static str = "stream_tags";
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::collections::HashMap;

    #[test]
    fn test_new_command() {
        let names = vec!["tag1".to_string(), "tag2".to_string()];
        let cmd = StreamTagsCommand::new(names.clone());
        assert_eq!(cmd.stream_tags, names);
    }

    #[test]
    fn test_command_serialization() {
        let cmd = StreamTagsCommand::new(vec!["a".to_string(), "b".to_string()]);
        let json = serde_json::to_string(&cmd).expect("Serialization failed");

        let deserialized: StreamTagsCommand =
            serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(deserialized.stream_tags, cmd.stream_tags);

        let expected_json = r#"{"stream_tags":["a","b"]}"#;
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_command_deserialization() {
        let json = r#"{"stream_tags":["x","y","z"]}"#;
        let cmd: StreamTagsCommand = serde_json::from_str(json).expect("Deserialization failed");
        assert_eq!(cmd.stream_tags, vec!["x", "y", "z"]);
    }

    #[test]
    fn test_response_serialization() {
        let mut map = HashMap::new();
        map.insert(
            "group1".to_string(),
            vec!["t1".to_string(), "t2".to_string()],
        );
        map.insert("group2".to_string(), vec!["t3".to_string()]);
        let response = StreamTagsCommandResponse { stream_tags: map };
        let json = serde_json::to_string(&response).expect("Serialization failed");
        let deserialized: StreamTagsCommandResponse =
            serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(deserialized.stream_tags, response.stream_tags);
        let expected_json = r#"{"stream_tags":{"group1":["t1","t2"],"group2":["t3"]}}"#;
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{"stream_tags":{"g":["v1","v2"]}}"#;
        let response: StreamTagsCommandResponse =
            serde_json::from_str(json).expect("Deserialization failed");
        let mut expected_map = HashMap::new();
        expected_map.insert("g".to_string(), vec!["v1".to_string(), "v2".to_string()]);
        assert_eq!(response.stream_tags, expected_map);
    }

    #[test]
    fn test_command_name_constant() {
        assert_eq!(StreamTagsCommand::NAME, "stream_tags");
    }

    #[test]
    fn test_command_associated_type() {
        fn assert_response_type<T: MistCommand<Response = StreamTagsCommandResponse>>() {}
        assert_response_type::<StreamTagsCommand>();
    }
}
