//! Core API client for the Mist server.
//!
//! This module provides the low-level `MistApi` client that handles
//! sending serializable commands to the Mist API and deserializing
//! the responses. It uses HTTP GET requests with a `command` query
//! parameter containing the JSON‑encoded command.

use crate::Result;
use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};
use url::Url;

/// Low‑level API client for the Mist server.
///
/// Wraps an HTTP client and the base API URL. Provides a generic `send`
/// method that serializes a command, sends it as a GET request with the
/// `command` query parameter, and deserializes the response.
#[derive(Debug, Clone, Default)]
pub struct MistApi {
    mist_api_url: String,
    client: Client,
}

impl MistApi {
    /// Creates a new `MistApi` instance.
    ///
    /// # Arguments
    /// * `mist_api_url` - The base URL of the Mist API (e.g., `http://localhost:4242/api`).
    /// * `client` - A shared HTTP client wrapped in an `Arc`.
    pub(crate) fn new(mist_api_url: String, client: Client) -> Self {
        Self {
            mist_api_url,
            client,
        }
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
    ///
    /// # Example
    /// ```no_run
    /// # use mistserver_rs::MistApi;
    /// # use std::sync::Arc;
    /// # use reqwest::Client;
    /// # use serde::{Serialize, Deserialize};
    /// #[derive(Serialize)]
    /// struct MyCommand { param: String }
    /// #[derive(Deserialize)]
    /// struct MyResponse { result: String }
    ///
    /// let api = MistApi::new(
    ///     "http://localhost:4242/api".to_string(),
    ///     Arc::new(Client::new()),
    /// );
    /// let response: MyResponse = api.send(MyCommand { param: "value".into() }).await?;
    /// # Ok::<(), mistserver_rs::MistError>(())
    /// ```
    ///
    /// **Note:** The current implementation sends two identical requests and
    /// returns the second response. The first request’s result is ignored.
    /// This is a known issue and may be fixed in future versions.
    pub(crate) async fn send<T, C>(&self, command: C) -> Result<T>
    where
        T: Send + Sync + DeserializeOwned,
        C: Send + Sync + Serialize,
    {
        let mut request_url = Url::parse(&self.mist_api_url)?;
        let command = serde_json::to_string(&command)?;

        request_url
            .query_pairs_mut()
            .append_pair("command", &command);

        let response = self.client.get(request_url).send().await?;

        Ok(response.json::<T>().await?)
    }
}
