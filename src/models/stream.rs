use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{MistError, Result};

/// A stream configuration object, sent in `addstream` / `streams` API calls.
///
/// # Required
/// - `source` – must always be set.
///
/// All other fields are optional; the server uses defaults when omitted.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    /// **Required.** The source of the media.
    ///
    /// Can be a file path, `push://`, `dtsc://`, RTSP URL, etc.
    pub source: String,

    /// **Required.** The name of the stream
    ///
    /// This will be generated automatically if you not specify in builder.
    #[serde(skip_serializing, skip_deserializing)]
    pub name: String,

    /// If `true`, keep the stream active even with no viewers.
    /// Avoids startup delay for the first viewer.
    /// Default: `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_on: Option<bool>,

    /// Buffer duration in milliseconds for live streams.
    /// Controls how far back viewers can seek.
    /// Default: `50000` (50 sec).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_time: Option<i32>,

    /// Debug verbosity level (1–6).
    /// - `3` = production default
    /// - lower = less logging
    /// - higher = more detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<i32>,

    /// If this stream cannot be opened (e.g. source offline),
    /// redirect requests to this stream name.
    /// Supports variable substitution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_stream: Option<String>,

    /// Holds any additional protocol‑specific parameters
    /// (e.g. `username`, `password`, `cut_time`, etc.).
    /// The server will accept them if they are valid for the source type.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

impl Stream {}

/// Detailed information about a stream, returned by the server in responses
/// (e.g. after `addstream` or when listing streams).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    /// The stream name (same as the key in the parent map).
    pub name: String,

    /// The configured source URI.
    pub source: String,

    /// Current Debug level only for this stream
    pub debug: Option<i32>,

    /// Status of always on for this stream
    #[serde(rename = "alwaysOn")]
    pub always_on: Option<bool>,

    /// Human‑readable status (e.g. `"Available"` for VoD, or an error message).
    pub error: Option<String>,

    /// Online state: `0` = error, `1` = active, `2` = inactive (§7.1.4).
    pub online: Option<i32>,

    /// Any extra metadata the server may include (e.g. `bufferTime`, `alwaysOn`).
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Fluent builder for creating a `Stream` configuration.
#[derive(Default, Debug)]
pub struct StreamBuilder {
    source: String,
    name: String,
    always_on: Option<bool>,
    buffer_time: Option<i32>,
    debug: Option<i32>,
    fallback_stream: Option<String>,
    extra: Option<HashMap<String, serde_json::Value>>,
}

impl StreamBuilder {
    /// Start with a mandatory `source` (required).
    pub fn new(name: &str, source: &str) -> Self {
        Self {
            source: source.into(),
            name: name.into(),
            ..Default::default()
        }
    }

    /// [Optional] Set a name for the stream (not actually used by the API,
    /// included for consistency; the name is the key in the `HashMap` when adding streams).
    pub fn name(mut self, value: &'static str) -> Self {
        self.name = value.into();
        self
    }

    /// Keep the stream active even with no viewers (default: `false`).
    pub fn always_on(mut self, value: bool) -> Self {
        self.always_on = Some(value);
        self
    }

    /// Buffer duration in milliseconds for live streams (default: `50000`).
    pub fn buffer_time(mut self, value: i32) -> Self {
        self.buffer_time = Some(value);
        self
    }

    /// Debug verbosity level (1‑6, default: `3`).
    pub fn debug(mut self, value: i32) -> Self {
        self.debug = Some(value);
        self
    }

    /// Fallback stream name if this one cannot be opened.
    pub fn fallback_stream(mut self, value: &'static str) -> Self {
        self.fallback_stream = Some(value.into());
        self
    }

    /// Add any extra parameters (e.g. `username`, `password`, `cut_time`).
    pub fn extra(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.extra = Some(value);
        self
    }

    fn validate_stream_name(&self, name: &str) -> Result<()> {
        if !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(MistError::InvalidStreamName(name.into()));
        }

        Ok(())
    }

    /// Build the final `Stream` object.
    pub fn build(self) -> Result<Stream> {
        self.validate_stream_name(&self.name)?;

        Ok(Stream {
            name: self.name,
            source: self.source,
            always_on: self.always_on,
            buffer_time: self.buffer_time,
            debug: self.debug,
            fallback_stream: self.fallback_stream,
            extra: self.extra,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    // ------------------------------------------------------------------------
    // Builder tests
    // ------------------------------------------------------------------------

    #[test]
    fn builder_defaults() -> Result<()> {
        let stream = StreamBuilder::new("name", "push://").build()?;
        assert_eq!(stream.source, "push://");
        assert!(stream.always_on.is_none());
        assert!(stream.buffer_time.is_none());
        assert!(stream.debug.is_none());
        assert!(stream.fallback_stream.is_none());
        assert!(stream.extra.is_none());

        Ok(())
    }

    #[test]
    fn builder_set_all_fields() -> Result<()> {
        let mut extra = HashMap::new();
        extra.insert("cut_time".to_string(), json!(0));
        extra.insert("segment_size".to_string(), json!(6000));

        let stream = StreamBuilder::new("name", "dtsc://1.2.3.4/video")
            .name("ignored")
            .always_on(true)
            .buffer_time(30000)
            .debug(4)
            .fallback_stream("backup")
            .extra(extra.clone())
            .build()?;

        assert_eq!(stream.source, "dtsc://1.2.3.4/video");
        assert_eq!(stream.always_on, Some(true));
        assert_eq!(stream.buffer_time, Some(30000));
        assert_eq!(stream.debug, Some(4));
        assert_eq!(stream.fallback_stream, Some("backup".to_string()));
        assert_eq!(stream.extra, Some(extra));

        Ok(())
    }

    #[test]
    fn builder_partial_fields() -> Result<()> {
        let stream = StreamBuilder::new("name", "file:///media/video.mp4")
            .always_on(false)
            .debug(3)
            .build()?;

        assert_eq!(stream.source, "file:///media/video.mp4");
        assert_eq!(stream.always_on, Some(false));
        assert!(stream.buffer_time.is_none());
        assert_eq!(stream.debug, Some(3));
        assert!(stream.fallback_stream.is_none());
        assert!(stream.extra.is_none());

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Serialization tests (Stream -> JSON)
    // ------------------------------------------------------------------------

    #[test]
    fn serialize_stream_all_fields() {
        let mut extra = HashMap::new();
        extra.insert("custom".to_string(), json!("value"));

        let stream = Stream {
            name: "".into(),
            source: "push://".to_string(),
            always_on: Some(true),
            buffer_time: Some(50000),
            debug: Some(3),
            fallback_stream: Some("fallback".to_string()),
            extra: Some(extra),
        };

        let value = serde_json::to_value(&stream).unwrap();
        let expected = json!({
            "source": "push://",
            "alwaysOn": true,
            "bufferTime": 50000,
            "debug": 3,
            "fallbackStream": "fallback",
            "custom": "value"
        });
        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_stream_only_required() {
        let stream = Stream {
            name: "".into(),
            source: "file:///movie.mp4".to_string(),
            always_on: None,
            buffer_time: None,
            debug: None,
            fallback_stream: None,
            extra: None,
        };

        let value = serde_json::to_value(&stream).unwrap();
        let expected = json!({
            "source": "file:///movie.mp4"
        });
        assert_eq!(value, expected);
    }

    #[test]
    fn serialize_stream_with_extra_only() {
        let mut extra = HashMap::new();
        extra.insert("username".to_string(), json!("admin"));
        extra.insert("password".to_string(), json!("secret"));

        let stream = Stream {
            name: "".into(),
            source: "rtsp://host/path".to_string(),
            always_on: None,
            buffer_time: None,
            debug: None,
            fallback_stream: None,
            extra: Some(extra),
        };

        let value = serde_json::to_value(&stream).unwrap();
        let expected = json!({
            "source": "rtsp://host/path",
            "username": "admin",
            "password": "secret"
        });
        assert_eq!(value, expected);
    }

    #[test]
    fn deserialize_stream_info() {
        let json_data = json!({
            "name": "mystream",
            "source": "push://",
            "error": "Available",
            "online": 2,
            "alwaysOn": true,
            "bufferTime": 30000,
            "extraField": "some value"
        });

        let info: StreamInfo = serde_json::from_value(json_data).unwrap();
        assert_eq!(info.name, "mystream");
        assert_eq!(info.error, Some("Available".into()));
        assert_eq!(info.online, Some(2 as i32));
        assert_eq!(info.source, "push://");
        assert_eq!(info.extra["alwaysOn"], true);
        assert_eq!(info.extra["bufferTime"], 30000);
        assert_eq!(info.extra["extraField"], "some value");
    }

    #[test]
    fn deserialize_stream_info_minimal() {
        let json_data = json!({
            "name": "vod",
            "source": "/media/video.mp4",
            "error": "Available",
            "online": 2
        });

        let info: StreamInfo = serde_json::from_value(json_data).unwrap();
        assert_eq!(info.name, "vod");
        assert_eq!(info.source, "/media/video.mp4");
        assert_eq!(info.error, Some("Available".into()));
        assert_eq!(info.online, Some(2 as i32));
        assert!(info.extra.is_empty());
    }

    // This test verifies that a Stream can also be deserialized (if needed).
    #[test]
    fn deserialize_stream() {
        let json_data = json!({
            "source": "push://",
            "alwaysOn": true,
            "bufferTime": 60000,
            "debug": 5,
            "fallbackStream": "backup",
            "customOption": "anything"
        });

        let stream: Stream = serde_json::from_value(json_data).unwrap();
        assert_eq!(stream.source, "push://");
        assert_eq!(stream.always_on, Some(true));
        assert_eq!(stream.buffer_time, Some(60000));
        assert_eq!(stream.debug, Some(5));
        assert_eq!(stream.fallback_stream, Some("backup".to_string()));
        assert!(stream.extra.is_some());
        let extra = stream.extra.unwrap();
        assert_eq!(extra["customOption"], "anything");
    }

    // ------------------------------------------------------------------------
    // Round‑trip tests (serialize then deserialize)
    // ------------------------------------------------------------------------

    #[test]
    fn roundtrip_stream() {
        let mut extra = HashMap::new();
        extra.insert("secret".to_string(), json!("1234"));

        let original = Stream {
            name: "".into(),
            source: "dtsc://server/stream".to_string(),
            always_on: Some(false),
            buffer_time: Some(40000),
            debug: Some(2),
            fallback_stream: Some("alt".to_string()),
            extra: Some(extra),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: Stream = serde_json::from_value(serialized).unwrap();

        assert_eq!(deserialized.source, original.source);
        assert_eq!(deserialized.always_on, original.always_on);
        assert_eq!(deserialized.buffer_time, original.buffer_time);
        assert_eq!(deserialized.debug, original.debug);
        assert_eq!(deserialized.fallback_stream, original.fallback_stream);
        let original_extra = original.extra.unwrap();
        let deserialized_extra = deserialized.extra.unwrap();
        assert_eq!(deserialized_extra, original_extra);
    }
}
