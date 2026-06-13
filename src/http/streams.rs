//! Stream management controller for the Mist API.
//!
//! This module provides functionality to manage streams, including creating
//! and deleting streams via the Mist API. It defines the command structures
//! and the controller that interacts with the API.

use crate::{
    Result,
    commands::streams::StreamAddCommand,
    http::MistApi,
    models::{Stream, StreamInfo},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

/// Response received after successfully adding streams.
///
/// Contains a map of stream names to their detailed information as returned
/// by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamAddResponse {
    /// Map of stream names to their `StreamInfo` details.
    pub streams: HashMap<String, StreamInfo>,
}

/// Command for deleting one or more streams.
///
/// The Mist API accepts multiple formats for deletion:
/// - A single stream name as a string.
/// - An array of stream names.
/// - A more complex object (hash map) for advanced deletion criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteStreamCommand {
    /// Delete a single stream by name.
    Single(String),
    /// Delete multiple streams by their names.
    Array(Vec<String>),
    /// Delete streams using a complex object (e.g., with filters).
    Complex(HashMap<String, serde_json::Value>),
}

/// Controller for managing streams via the Mist API.
///
/// Provides methods to perform operations on streams such as creating new ones.
pub struct StreamsController {
    api: Arc<MistApi>,
}

impl StreamsController {
    /// Creates a new `StreamsController` with the given API handle.
    pub fn new(api: Arc<MistApi>) -> Self {
        Self { api }
    }

    /// Creates multiple streams in a single API call.
    ///
    /// # Arguments
    /// * `streams` - A map from stream names to their `Stream` configurations.
    ///
    /// # Returns
    /// A `Result` containing the `StreamAddResponse` with details of the created streams.
    pub async fn create(&self, streams: HashMap<String, Stream>) -> Result<StreamAddResponse> {
        let command = StreamAddCommand { addstream: streams };
        let response: StreamAddResponse = self.api.send(command).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use mockito::{Matcher, Server, ServerOpts};
    use reqwest::Client;
    use serde_json::json;
    use std::time::Duration;

    use super::*;
    use crate::utils::build_http_client;

    fn test_client() -> Client {
        build_http_client(Duration::from_secs(10)).expect("failed to build test client")
    }

    fn sample_stream(source: &str) -> Stream {
        Stream {
            source: source.into(),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn create_multiple_streams_success() -> Result<()> {
        let mut server = Server::new_with_opts_async(ServerOpts {
            host: "0.0.0.0",
            port: 1234,
            ..Default::default()
        })
        .await;

        let api_url = "http://localhost:1234/api".to_string();
        let client = test_client();

        let api = Arc::new(MistApi::new(api_url, client));
        let stream_ctrl = StreamsController::new(api);
        let mut streams = HashMap::new();

        streams.insert("stream1".to_string(), sample_stream("push://a"));
        streams.insert("stream2".to_string(), sample_stream("file://b.mp4"));

        let expected_command = json!({
            "addstream": {
                "stream1": { "source": "push://a" },
                "stream2": { "source": "file://b.mp4" }
            }
        });

        let response_body = json!({
            "streams": {
                "stream1": { "name": "stream1", "source": "push://a", "error": "Available", "online": 2 },
                "stream2": { "name": "stream2", "source": "file://b.mp4", "error": "Available", "online": 2 },
            }
        });

        let mock = server
            .mock("GET", "/api")
            .match_query(Matcher::Any)
            .with_status(200)
            .with_body(response_body.to_string())
            .create();

        let response = stream_ctrl.create(streams).await?;

        assert_eq!(response.streams.len(), 2);
        assert!(response.streams.contains_key("stream1"));
        assert!(response.streams.contains_key("stream2"));
        mock.assert();

        Ok(())
    }
}
