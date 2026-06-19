//! Mist API client library.
//!
//! This module provides a client for interacting with the Mist API,
//! including optional authentication and builder-based configuration.

use std::sync::Arc;

use crate::{
    Result,
    http::{AuthResult, MistApi, MistApiBuilder, MistAuthController, StreamsController},
};
use reqwest::Client;

/// A client for interacting with the Mist API.
///
/// Holds the base API URL, authentication credentials (if any), an HTTP client,
/// and an optional authentication controller for handling authentication flows.
#[derive(Debug, Clone)]
pub struct MistClient {
    pub(crate) auth: Option<(String, String)>,
    pub(crate) auth_controller: Option<MistAuthController>,
    pub(crate) auth_result: Option<AuthResult>,
    pub(crate) api: Arc<MistApi>,
}

impl MistClient {
    pub async fn authorize(&mut self) -> Result<()> {
        let Some(controller) = &self.auth_controller else {
            return Ok(());
        };

        let auth_result = controller.authorize().await?;
        self.auth_result = Some(auth_result);

        Ok(())
    }

    pub async fn streams(&self) -> StreamsController {
        StreamsController::new(self.api.clone())
    }

    /// Returns `true` if authentication credentials have been set.
    pub fn auth_enabled(&self) -> bool {
        self.auth.is_some()
    }

    /// Returns a copy of the authentication credentials, if present.
    pub fn auth_credentials(&self) -> Option<(String, String)> {
        self.auth.clone()
    }
}

/// Builder for creating a [`MistClient`] with custom configuration.
///
/// The builder requires an HTTP client to be provided via [`with_client`] before
/// calling [`build`]; it will panic otherwise.
///
/// # Example
/// ```
/// use std::sync::Arc;
/// use reqwest::Client;
/// use mist_client::MistClientBuilder;
///
/// let client = Arc::new(Client::new());
/// let mist_client = MistClientBuilder::new("https://api.mist.com")
///     .with_client(client)
///     .with_auth("user", "pass")
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct MistClientBuilder {
    pub mist_api_url: String,
    pub client: Client,
    pub auth: Option<(String, String)>,
}

impl MistClientBuilder {
    /// Creates a new builder with the given base API URL.
    ///
    /// No client or authentication is set initially.
    pub fn new(base_api_url: &str) -> Self {
        Self {
            auth: None,
            mist_api_url: base_api_url.into(),
            client: Client::new(),
            ..Default::default()
        }
    }

    /// Sets the authentication credentials for the client.
    ///
    /// Both username and password are stored as strings.
    pub fn with_auth(mut self, username: &str, password: &str) -> Self {
        self.auth = Some((username.into(), password.into()));
        self
    }

    /// Sets the HTTP client to be used by the constructed [`MistClient`].
    ///
    /// The client is wrapped in an `Arc` and shared.
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = client;
        self
    }

    /// Builds the final [`MistClient`].
    ///
    /// # Panics
    /// Panics if no HTTP client has been provided via [`with_client`].
    pub fn build(self) -> MistClient {
        let auth = self.auth;
        let client = self.client;

        let auth_controller = auth.as_ref().map(|_| {
            MistAuthController::new(client.clone(), self.mist_api_url.clone(), auth.clone())
        });

        let api = Arc::new(
            MistApiBuilder::new()
                .with_client(client.clone())
                .with_url(self.mist_api_url)
                .build(),
        );

        MistClient {
            api,
            auth_controller,
            auth,
            auth_result: None,
        }
    }
}
