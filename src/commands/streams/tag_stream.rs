use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Command for assigning tags to one or more streams.
///
/// This command serializes to the `tag_stream` payload expected by the MistServer
/// API. Each stream can be associated with either a single tag or multiple tags.
///
/// # JSON representation
///
/// ```json
/// {
///   "tag_stream": {
///     "stream1": "live",
///     "stream2": ["sports", "premium"]
///   }
/// }
/// ```
///
/// # Example
///
/// ```
/// # use std::collections::HashMap;
/// # use your_crate::{TagStreamCommand, TagStreamCommandValue};
///
/// let mut streams = HashMap::new();
///
/// streams.insert(
///     "stream1".into(),
///     TagStreamCommandValue::from("live"),
/// );
///
/// streams.insert(
///     "stream2".into(),
///     TagStreamCommandValue::from(vec![
///         "sports".to_string(),
///         "premium".to_string(),
///     ]),
/// );
///
/// let command = TagStreamCommand::new(streams);
/// ```
///
/// # Notes
///
/// - Stream names are not validated by this type.
/// - Empty tag lists are allowed but may be rejected by the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagStreamCommand {
    tag_stream: HashMap<String, TagValue>,
}

impl TagStreamCommand {
    /// Creates a new `TagStreamCommand`.
    ///
    /// # Arguments
    ///
    /// * `hash_map` - A mapping of stream names to one or more tags.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use your_crate::{TagStreamCommand, TagStreamCommandValue};
    ///
    /// let command = TagStreamCommand::new(HashMap::from([
    ///     (
    ///         "stream".to_string(),
    ///         TagStreamCommandValue::from("live"),
    ///     ),
    /// ]));
    /// ```
    #[must_use]
    pub fn new(hash_map: HashMap<String, TagValue>) -> Self {
        Self {
            tag_stream: hash_map,
        }
    }
}

/// Tags assigned to a stream.
///
/// A stream can have either a single tag or multiple tags. The
/// `serde(untagged)` attribute allows the enum to serialize directly to the
/// JSON format expected by the API.
///
/// # JSON representation
///
/// A single tag:
///
/// ```json
/// "live"
/// ```
///
/// Multiple tags:
///
/// ```json
/// ["sports", "premium"]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TagValue {
    /// A single tag.
    Single(String),

    /// Multiple tags.
    Multiple(Vec<String>),
}

impl From<&str> for TagValue {
    /// Creates a single-tag value from a string slice.
    fn from(value: &str) -> Self {
        Self::Single(value.into())
    }
}

impl From<Vec<String>> for TagValue {
    /// Creates a multi-tag value from a vector of strings.
    fn from(value: Vec<String>) -> Self {
        Self::Multiple(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_from_str() {
        let val = TagValue::from("hello");
        assert!(matches!(val, TagValue::Single(s) if s == "hello"));
    }

    #[test]
    fn test_from_vec() {
        let vec = vec!["a".to_string(), "b".to_string()];
        let val = TagValue::from(vec.clone());
        assert!(matches!(val, TagValue::Multiple(v) if v == vec));
    }

    #[test]
    fn test_serialize_single() {
        let mut map = HashMap::new();
        map.insert("stream".to_string(), TagValue::from("tag"));
        let cmd = TagStreamCommand::new(map);
        let json = serde_json::to_string(&cmd).unwrap();
        // Should produce {"tag_stream":{"stream":"tag"}}
        assert_eq!(json, r#"{"tag_stream":{"stream":"tag"}}"#);
    }

    #[test]
    fn test_serialize_multiple() {
        let mut map = HashMap::new();
        map.insert(
            "stream".to_string(),
            TagValue::from(vec!["a".to_string(), "b".to_string()]),
        );
        let cmd = TagStreamCommand::new(map);
        let json = serde_json::to_string(&cmd).unwrap();
        assert_eq!(json, r#"{"tag_stream":{"stream":["a","b"]}}"#);
    }

    #[test]
    fn test_empty_map() {
        let map = HashMap::new();
        let cmd = TagStreamCommand::new(map);
        let json = serde_json::to_string(&cmd).unwrap();
        assert_eq!(json, r#"{"tag_stream":{}}"#);
        let deserialized: TagStreamCommand = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.tag_stream.len(), 0);
    }

    #[test]
    fn test_empty_vec() {
        let mut map = HashMap::new();
        map.insert("stream".to_string(), TagValue::from(Vec::<String>::new()));
        let cmd = TagStreamCommand::new(map);
        let json = serde_json::to_string(&cmd).unwrap();
        assert_eq!(json, r#"{"tag_stream":{"stream":[]}}"#);
        // Deserialize back
        let deserialized: TagStreamCommand = serde_json::from_str(&json).unwrap();
        match deserialized.tag_stream.get("stream").unwrap() {
            TagValue::Multiple(v) => assert!(v.is_empty()),
            _ => panic!("Expected Multiple variant"),
        }
    }

    #[test]
    fn test_multiple_streams() {
        let mut map = HashMap::new();
        map.insert("a".to_string(), TagValue::from("tag1"));
        map.insert(
            "b".to_string(),
            TagValue::from(vec!["tag2".to_string(), "tag3".to_string()]),
        );
        let cmd = TagStreamCommand::new(map);
        let json = serde_json::to_string(&cmd).unwrap();
        // We don't check exact string because hashmap order is not guaranteed; we check it deserializes correctly.
        let deserialized: TagStreamCommand = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.tag_stream.len(), 2);
        assert!(deserialized.tag_stream.contains_key("a"));
        assert!(deserialized.tag_stream.contains_key("b"));
        match deserialized.tag_stream.get("a").unwrap() {
            TagValue::Single(s) => assert_eq!(s, "tag1"),
            _ => panic!("Expected Single"),
        }
        match deserialized.tag_stream.get("b").unwrap() {
            TagValue::Multiple(v) => {
                assert_eq!(v, &vec!["tag2".to_string(), "tag3".to_string()])
            }
            _ => panic!("Expected Multiple"),
        }
    }
}
