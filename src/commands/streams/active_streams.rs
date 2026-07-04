//! Active stream statistics command.
//!
//! This module provides the command used to retrieve real-time statistics for
//! all currently active streams. The Mist API returns stream metrics as
//! positional arrays rather than named objects, so the response types use tuple
//! deserialization to map each value into a strongly typed Rust structure.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_tuple::Deserialize_tuple;

use crate::commands::traits::MistCommand;

/// Command for retrieving statistics about all active streams.
///
/// The command requests a predefined set of metrics from the Mist API.
/// Each metric name corresponds to a value returned in the positional
/// response array for every active stream.
#[derive(Debug, Clone, Serialize)]
pub struct ListActiveStreamsCommand {
    active_streams: Vec<&'static str>,
}

impl ListActiveStreamsCommand {
    /// Creates a new command requesting the default set of stream metrics
    /// supported by the SDK.
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

/// Response returned by the `active_streams` command.
///
/// The response maps stream names to their current runtime statistics.
/// When no streams are active, the field may be absent.
#[derive(Debug, Clone, Deserialize)]
pub struct ListActiveStreamsResponse {
    active_streams: Option<HashMap<String, ActiveStreamStats>>,
}

/// Runtime statistics describing a single active stream.
///
/// The Mist API represents these values as a positional array rather than a
/// JSON object. The field order therefore matches the order of the requested
/// metrics in [`ListActiveStreamsCommand`].
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

/// Detailed health information reported for an individual media track.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackInfo {
    pub bframes: Option<bool>,
    pub buffer: Option<i32>,
    pub codec: Option<String>,
    pub efpks: Option<i32>,
    pub efps: Option<f32>,
    pub fpks: Option<i32>,
    pub fps: Option<i32>,
    pub height: Option<i32>,
    pub id: Option<i32>,
    pub idx: Option<i32>,
    pub jitter: Option<i32>,
    pub kbits: Option<i32>,
    pub keys: Option<KeyInfo>,
    pub width: Option<i32>,
}

/// Keyframe timing and interval statistics for a media track.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KeyInfo {
    pub frame_ms_max: Option<i32>,
    pub frame_ms_min: Option<i32>,
    pub frames_max: Option<i32>,
    pub frames_min: Option<i32>,
    pub ms_max: Option<i32>,
    pub ms_min: Option<i32>,
}

/// Health diagnostics for an active stream.
///
/// Besides general stream health metrics, the response contains a dynamic set
/// of track-specific entries. Those entries are collected into
/// `tracks_detail` using Serde's `flatten` support.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActiveStreamHealth {
    pub buffer: Option<i32>,
    pub issues: Option<String>,
    pub jitter: Option<i32>,
    pub tracks: Option<Vec<String>>,

    #[serde(flatten)]
    pub tracks_detail: Option<HashMap<String, TrackInfo>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::traits::MistCommand;
    use serde_json::{Value, json};

    #[test]
    fn list_active_streams_command_new_contains_all_default_metrics() {
        let command = ListActiveStreamsCommand::new();

        let value = serde_json::to_value(&command).unwrap();

        let metrics = value["active_streams"].as_array().unwrap();

        assert_eq!(metrics.len(), 15);

        let expected = [
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
        ];

        for metric in expected {
            assert!(metrics.contains(&Value::String(metric.to_string())));
        }
    }

    #[test]
    fn list_active_streams_command_serialization() {
        let command = ListActiveStreamsCommand::new();

        let expected = json!({
            "active_streams": [
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
                "health"
            ]
        });

        let actual = serde_json::to_value(command).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn list_active_streams_command_name() {
        assert_eq!(ListActiveStreamsCommand::NAME, "active_streams");
    }

    #[test]
    fn list_active_streams_response_deserializes_empty_object() {
        let json = json!({
            "active_streams": {}
        });

        let _: ListActiveStreamsResponse =
            serde_json::from_value(json).expect("response should deserialize");
    }

    #[test]
    fn list_active_streams_response_deserializes_null() {
        let json = json!({
            "active_streams": null
        });

        let _: ListActiveStreamsResponse =
            serde_json::from_value(json).expect("response should deserialize");
    }

    #[test]
    fn list_active_streams_response_deserializes_single_stream() {
        let json = json!({
            "active_streams": {
                "camera1": [
                    5,
                    1500,
                    100,
                    3,
                    1,
                    2,
                    40,
                    900,
                    100000,
                    200000,
                    3000,
                    5,
                    2,
                    "active",
                    {
                        "buffer": 25,
                        "issues": "",
                        "jitter": 1,
                        "tracks": [
                            "video"
                        ],
                        "video": {
                            "bframes": false,
                            "buffer": 12,
                            "codec": "H264",
                            "efpks": 0,
                            "efps": 30,
                            "fpks": 0,
                            "fps": 30,
                            "height": 1080,
                            "id": 1,
                            "idx": 0,
                            "jitter": 0,
                            "kbits": 4500,
                            "keys": {
                                "frame_ms_max": 33,
                                "frame_ms_min": 33,
                                "frames_max": 30,
                                "frames_min": 30,
                                "ms_max": 1000,
                                "ms_min": 1000
                            },
                            "width": 1920
                        }
                    }
                ]
            }
        });

        let _: ListActiveStreamsResponse =
            serde_json::from_value(json).expect("response should deserialize");
    }

    #[test]
    fn active_stream_health_deserializes_dynamic_tracks() {
        let json = json!({
            "buffer": 20,
            "issues": "",
            "jitter": 2,
            "tracks": [
                "video",
                "audio"
            ],
            "video": {
                "bframes": false,
                "buffer": 10,
                "codec": "H264",
                "efpks": 0,
                "efps": 30,
                "fpks": 0,
                "fps": 30,
                "height": 1080,
                "id": 1,
                "idx": 0,
                "jitter": 0,
                "kbits": 4000,
                "keys": {
                    "frame_ms_max": 33,
                    "frame_ms_min": 33,
                    "frames_max": 30,
                    "frames_min": 30,
                    "ms_max": 1000,
                    "ms_min": 1000
                },
                "width": 1920
            },
            "audio": {
                "bframes": false,
                "buffer": 4,
                "codec": "AAC",
                "efpks": 0,
                "efps": 50,
                "fpks": 0,
                "fps": 50,
                "height": 0,
                "id": 2,
                "idx": 1,
                "jitter": 0,
                "kbits": 128,
                "keys": {
                    "frame_ms_max": 20,
                    "frame_ms_min": 20,
                    "frames_max": 50,
                    "frames_min": 50,
                    "ms_max": 1000,
                    "ms_min": 1000
                },
                "width": 0
            }
        });

        let health: ActiveStreamHealth = serde_json::from_value(json).unwrap();

        assert_eq!(health.buffer, Some(20));
        assert_eq!(health.jitter, Some(2));
        assert_eq!(health.tracks.len(), 2);

        assert!(health.tracks_detail.contains_key("video"));
        assert!(health.tracks_detail.contains_key("audio"));

        assert_eq!(health.tracks_detail["video"].codec, "H264");
        assert_eq!(health.tracks_detail["audio"].codec, "AAC");
    }

    #[test]
    fn track_info_roundtrip() {
        let json = json!({
            "bframes": false,
            "buffer": 10,
            "codec": "H264",
            "efpks": 0,
            "efps": 30,
            "fpks": 0,
            "fps": 30,
            "height": 720,
            "id": 1,
            "idx": 0,
            "jitter": 1,
            "kbits": 2500,
            "keys": {
                "frame_ms_max": 40,
                "frame_ms_min": 33,
                "frames_max": 30,
                "frames_min": 30,
                "ms_max": 1000,
                "ms_min": 1000
            },
            "width": 1280
        });

        let track: TrackInfo = serde_json::from_value(json.clone()).unwrap();

        let serialized = serde_json::to_value(track).unwrap();

        assert_eq!(serialized, json);
    }

    #[test]
    fn key_info_roundtrip() {
        let json = json!({
            "frame_ms_max": 40,
            "frame_ms_min": 33,
            "frames_max": 30,
            "frames_min": 30,
            "ms_max": 1000,
            "ms_min": 1000
        });

        let keys: KeyInfo = serde_json::from_value(json.clone()).unwrap();
        let serialized = serde_json::to_value(keys).unwrap();

        assert_eq!(serialized, json);
    }
}
