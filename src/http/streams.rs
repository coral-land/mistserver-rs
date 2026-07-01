//! Stream management controller for the Mist API.
//!
//! This module provides functionality to manage streams, including creating
//! and deleting streams via the Mist API. It defines the command structures
//! and the controller that interacts with the API.

use crate::{
    MistError, Result, StreamInfo,
    commands::streams::{DeleteStreamCommand, StreamAddCommand},
    http::MistApi,
    models::Stream,
};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

/// Response received after successfully adding streams.
///
/// Contains a map of stream names to their detailed information as returned
/// by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamsEndpointResponse {
    /// Map of stream names to their `StreamInfo` details.
    #[serde(deserialize_with = "deserialize_streams_map")]
    pub streams: HashMap<String, StreamInfo>,
}

fn deserialize_streams_map<'de, D>(
    deserializer: D,
) -> std::result::Result<HashMap<String, StreamInfo>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let raw: HashMap<String, serde_json::Value> = HashMap::deserialize(deserializer)?;
    let mut result = HashMap::new();
    for (key, value) in raw {
        if key == "incomplete list" {
            continue;
        }
        let info: StreamInfo = serde_json::from_value(value).map_err(Error::custom)?;
        result.insert(key, info);
    }
    Ok(result)
}

/// Controller for managing streams via the Mist API.
///
/// Provides methods to perform operations on streams such as creating new ones.
pub struct StreamsApi {
    transport: Arc<MistApi>,
}

impl StreamsApi {
    /// Creates a new `StreamsController` with the given API handle.
    pub fn new(transport: Arc<MistApi>) -> Self {
        Self { transport }
    }

    /// Creates multiple streams in a single API call.
    ///
    /// # Arguments
    /// * `streams` - A map from stream names to their `Stream` configurations.
    ///
    /// # Returns
    /// A `Result` containing the `StreamAddResponse` with details of the created streams.
    async fn create(&self, streams: HashMap<String, Stream>) -> Result<StreamsEndpointResponse> {
        let command = StreamAddCommand { addstream: streams };
        let response = self.transport.send(command).await?;
        Ok(response)
    }

    /// Create one single stream
    /// This will update if the stream with same name exists based on the mist server api
    ///
    /// # Returns
    /// A `Result` containing the `StreamAddResponse` with details of the created streams.
    pub async fn add_stream(&self, stream: Stream) -> Result<StreamsEndpointResponse> {
        let mut streams_create_map = HashMap::new();
        streams_create_map.insert(stream.name.clone(), stream);

        let response = self.create(streams_create_map).await?;
        if response.streams.len() <= 0 {
            return Err(MistError::Api {
                message: "No streams returned in response, something broken".into(),
            });
        }

        Ok(response)
    }

    /// Creates many streams with your options
    /// This will update the stream with same name if exists.
    ///
    /// # Returns
    /// A `Result` containing the StreamAddResponse with details of created stream.
    pub async fn add_many_stream(
        &self,
        streams: HashMap<String, Stream>,
    ) -> Result<StreamsEndpointResponse> {
        let response = self.create(streams).await?;
        if response.streams.len() <= 0 {
            return Err(MistError::Api {
                message: "No streams returned in response, something broken".into(),
            });
        }

        Ok(response)
    }

    pub async fn delete_stream(&self, names: Vec<String>) -> Result<()> {
        let command = DeleteStreamCommand {
            deletestream: names,
        };

        let response: Option<HashMap<String, Value>> = self.transport.send(command).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::MistApi;
    use crate::utils::build_http_client;
    use mockito::{Matcher, Server};
    use reqwest::Client;
    use serde_json::json;
    use std::{collections::HashMap, sync::Arc, time::Duration};

    fn client() -> Client {
        build_http_client(Duration::from_secs(5)).unwrap()
    }

    fn stream(source: &str) -> Stream {
        Stream {
            source: source.into(),
            ..Default::default()
        }
    }

    async fn setup(response: serde_json::Value) -> StreamsApi {
        let mut server = Server::new_async().await;

        server
            .mock("GET", "/api")
            .match_query(Matcher::Any)
            .with_status(200)
            .with_body(response.to_string())
            .create();

        let url = format!("{}/api", server.url());
        std::mem::forget(server);

        StreamsApi::new(Arc::new(MistApi::new(url, client())))
    }

    #[tokio::test]
    async fn add_stream_success() {
        let api = setup(json!({
            "streams": {
                "camera": {
                    "name": "camera",
                    "source": "push://live",
                    "error": "Available",
                    "online": 1
                }
            }
        }))
        .await;

        let response = api.add_stream(stream("push://live")).await.unwrap();

        assert_eq!(response.streams.len(), 1);
        assert!(response.streams.contains_key("camera"));
    }

    #[tokio::test]
    async fn add_stream_returns_error_when_no_streams_returned() {
        let api = setup(json!({
            "streams": {}
        }))
        .await;

        let err = api.add_stream(stream("push://live")).await.unwrap_err();

        assert!(matches!(err, MistError::Api { .. }));
    }

    #[tokio::test]
    async fn ignores_incomplete_list_marker() {
        let api = setup(json!({
            "streams": {
                "incomplete list": true,
                "camera": {
                    "name": "camera",
                    "source": "push://live",
                    "error": "Available",
                    "online": 1
                }
            }
        }))
        .await;

        let response = api.add_stream(stream("push://live")).await.unwrap();

        assert_eq!(response.streams.len(), 1);
        assert!(response.streams.contains_key("camera"));
    }

    #[tokio::test]
    async fn add_many_streams_success() {
        let api = setup(json!({
            "streams": {
                "cam1": {
                    "name": "cam1",
                    "source": "push://1",
                    "error": "Available",
                    "online": 1
                },
                "cam2": {
                    "name": "cam2",
                    "source": "push://2",
                    "error": "Available",
                    "online": 1
                }
            }
        }))
        .await;

        let mut streams = HashMap::new();
        streams.insert("cam1".into(), stream("push://1"));
        streams.insert("cam2".into(), stream("push://2"));

        let response = api.add_many_stream(streams).await.unwrap();

        assert_eq!(response.streams.len(), 2);
    }

    #[tokio::test]
    async fn delete_stream_success() {
        let mut server = Server::new_async().await;

        server
            .mock("GET", "/api")
            .match_query(Matcher::Any)
            .with_status(200)
            .with_body("null")
            .create();

        let api = StreamsApi::new(Arc::new(MistApi::new(
            format!("{}/api", server.url()),
            client(),
        )));

        api.delete_stream(vec!["cam1".to_string(), "cam2".to_string()])
            .await
            .unwrap();
    }
}
