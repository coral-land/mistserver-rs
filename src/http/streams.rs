use crate::{
    Result,
    http::MistApi,
    models::{Stream, StreamInfo},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamAddCommand {
    addstream: HashMap<String, Stream>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamAddResponse {
    pub streams: HashMap<String, StreamInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteStreamCommand {
    Single(String),
    Array(Vec<String>),
    Complex(HashMap<String, serde_json::Value>),
}

pub struct StreamsController {
    api: Arc<MistApi>,
}

impl StreamsController {
    pub fn new(api: Arc<MistApi>) -> Self {
        Self { api }
    }

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

    fn test_client() -> Arc<Client> {
        Arc::new(build_http_client(Duration::from_secs(10)).expect("failed to build test client"))
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
            .expect(2)
            .create();

        let response = stream_ctrl.create(streams).await?;

        assert_eq!(response.streams.len(), 2);
        assert!(response.streams.contains_key("stream1"));
        assert!(response.streams.contains_key("stream2"));
        mock.assert();

        Ok(())
    }
}
