//! Core API client for the Mist server.
//!
//! This module provides the low-level `MistApi` client that handles
//! sending serializable commands to the Mist API and deserializing
//! the responses. It uses HTTP GET requests with a `command` query
//! parameter containing the JSON‑encoded command.

use crate::{MistError, Result, commands::traits::MistCommand};
use reqwest::Client;
use url::Url;

/// Low‑level API client for the Mist server.
///
/// Wraps an HTTP client and the base API URL. Provides a generic `send`
/// method that serializes a command, sends it as a GET request with the
/// `command` query parameter, and deserializes the response.
#[derive(Debug, Clone, Default)]
pub struct HttpTransport {
    api_url: String,
    client: Client,
}

impl HttpTransport {
    /// Creates a new `MistApi` instance.
    ///
    /// # Arguments
    /// * `mist_api_url` - The base URL of the Mist API (e.g., `http://localhost:4242/api`).
    /// * `client` - A shared HTTP client wrapped in an `Arc`.
    pub(crate) fn new(api_url: String, client: Client) -> Self {
        Self { api_url, client }
    }

    /// Sends a command to the Mist API and deserializes the response.
    ///
    /// The command is serialized to JSON and appended as a `command` query
    /// parameter to the base URL. The request is performed as an HTTP GET.
    ///
    /// # Type parameters
    /// * `T` - The expected response type, must be deserializable from JSON.
    /// * `C` - The command type, must be serializable to JSON.
    ///
    /// # Returns
    /// A `Result` containing the deserialized response of type `T`.
    pub(crate) async fn execute<C: MistCommand>(&self, command: C) -> Result<C::Response> {
        let mut request_url = Url::parse(&self.api_url)?;
        let command = serde_json::to_string(&command)?;

        tracing::debug!(command = %command, url = %request_url, "Sending API request");

        request_url
            .query_pairs_mut()
            .append_pair("command", &command);

        let response = self.client.get(request_url).send().await?;
        let response_text = response.text().await?;
        let json_value: serde_json::Value = serde_json::from_str(&response_text)?;
        let to_string = serde_json::to_string_pretty(&json_value)?;
        if let Some(error_msg) = json_value["error"].as_str() {
            return Err(MistError::Api {
                message: error_msg.into(),
            });
        }

        Ok(serde_json::from_value(json_value)?)
    }
}

/// Builder for constructing a [`MistApi`] instance with a fluent interface.
///
/// Allows customization of the HTTP client and the base API URL before
/// calling [`build`](ApiBuilder::build). If no customization is applied,
/// it defaults to `http://localhost:4242` and a new [`reqwest::Client`].
///
/// # Examples
///
/// ```no_run
/// # use reqwest::Client;
/// # use mistserver_rs::api::ApiBuilder;
/// let api = ApiBuilder::new()
///     .with_url("http://mist.example.com/api".into())
///     .with_client(Client::new())
///     .build();
/// ```
pub struct HttpTransportBuilder {
    client: Client,
    url: String,
}

impl HttpTransportBuilder {
    /// Creates a new `ApiBuilder` with default settings.
    ///
    /// Default URL: `http://localhost:4242`
    /// Default client: a newly constructed `reqwest::Client`.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            url: "http://localhost:4242".into(),
        }
    }

    /// Sets the HTTP client to be used by the `MistApi`.
    ///
    /// This is useful when you need to share a single client across
    /// multiple API instances, or when you want to configure
    /// custom TLS settings, timeouts, etc.
    pub fn with_client(mut self, client: Client) -> Self {
        self.client = client;
        self
    }

    /// Sets the base URL for the Mist API.
    ///
    /// The URL should *not* include a trailing slash or the `/api` path
    /// unless your Mist server expects it. The `send` method will
    /// append the `command` query parameter directly to this URL.
    pub fn with_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    /// Consumes the builder and returns a configured [`MistApi`].
    pub fn build(self) -> HttpTransport {
        HttpTransport::new(self.url, self.client)
    }
}

/// Default implementation for Mist Api Builder
///
impl Default for HttpTransportBuilder {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    /// Test-only command payload.
    #[derive(Debug, Serialize, PartialEq)]
    struct TestCommand {
        key: String,
    }

    impl MistCommand for TestCommand {
        type Response = TestResponse;
        const NAME: &'static str = "testcommand";
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct TestResponse {
        value: String,
    }

    #[test]
    fn builder_defaults() {
        let api = HttpTransportBuilder::new().build();
        assert_eq!(api.api_url, "http://localhost:4242");
    }

    #[test]
    fn builder_with_url() {
        let api = HttpTransportBuilder::new()
            .with_url("https://mist.example.com".into())
            .build();
        assert_eq!(api.api_url, "https://mist.example.com");
    }

    #[tokio::test]
    async fn send_command_ok() {
        let mut server = mockito::Server::new_async().await;
        let command = TestCommand {
            key: "my_key".into(),
        };
        let expected_response = TestResponse {
            value: "my_value".into(),
        };

        let expected_body = serde_json::to_string(&expected_response).unwrap();

        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![mockito::Matcher::UrlEncoded(
                "command".into(),
                serde_json::to_string(&command).unwrap(),
            )]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&expected_body)
            .create_async()
            .await;

        let api = HttpTransport::new(server.url(), Client::new());
        let response: TestResponse = api.execute(command).await.unwrap();

        assert_eq!(response, expected_response);
        _mock.assert_async().await;
    }

    #[tokio::test]
    async fn send_command_http_error() {
        let mut server = mockito::Server::new_async().await;
        let command = TestCommand { key: "bad".into() };

        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![mockito::Matcher::UrlEncoded(
                "command".into(),
                serde_json::to_string(&command).unwrap(),
            )]))
            .with_status(500)
            .create_async()
            .await;

        let api = HttpTransport::new(server.url(), Client::new());
        let result: Result<TestResponse> = api.execute(command).await;

        assert!(result.is_err());
        _mock.assert_async().await;
    }

    #[tokio::test]
    async fn send_command_invalid_json() {
        let mut server = mockito::Server::new_async().await;
        let command = TestCommand {
            key: "invalid_json".into(),
        };

        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![mockito::Matcher::UrlEncoded(
                "command".into(),
                serde_json::to_string(&command).unwrap(),
            )]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("not json")
            .create_async()
            .await;

        let api = HttpTransport::new(server.url(), Client::new());
        let result: Result<TestResponse> = api.execute(command).await;
        assert!(result.is_err());
        _mock.assert_async().await;
    }
}
