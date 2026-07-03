//! Mist API client library.
//!
//! This module provides a client for interacting with the Mist API,
//! including optional authentication and builder-based configuration.
//!
//! # Quick start
//! ```
//! use mist_client::{MistClient, MistClientBuilder};
//! use reqwest::Client;
//!
//! let client = Client::new();
//! let mist = MistClientBuilder::new("http://localhost:4242")
//!     .with_client(client)
//!     .with_auth("admin", "password")
//!     .build();
//!
//! ```

use std::sync::Arc;

use crate::{
    Result,
    commands::traits::MistCommand,
    controllers::{AuthController, AuthResult, StreamController},
    transport::{HttpTransport, HttpTransportBuilder},
};
use reqwest::Client;

/// A client for interacting with the Mist API.
///
/// Holds the base API URL, authentication credentials (if any), an HTTP client,
/// and an optional authentication controller for handling authentication flows.
///
/// # Fields
/// Most fields are internal; use the provided methods to interact with the API.
#[derive(Debug, Clone)]
pub struct MistClient {
    /// Optional plain‑text authentication credentials (username, password).
    pub(crate) auth: Option<(String, String)>,
    /// The result of the last authentication attempt (if any).
    pub(crate) auth_result: Option<AuthResult>,
    /// Shared API client that executes HTTP requests.
    pub(crate) transport: Arc<HttpTransport>,
}

impl MistClient {
    /// Returns auth controller witch you can use to authenticate
    /// before any other request if needed.
    /// Authorization needs to be performed only once.
    ///
    /// # Returns
    /// - An instance of `AuthController`.
    pub fn auth(&self) -> AuthController<'_> {
        AuthController::new(self.auth.clone(), self)
    }

    /// Returns a `StreamsController` for managing streams.
    ///
    /// This controller can be used to create, update, delete, and list streams.
    ///
    /// # Example
    /// ```
    /// # let client = MistClientBuilder::new("http://localhost:4242").build();
    /// let streams = client.streams().await;
    /// // streams.create(...).await?;
    /// ```
    pub fn streams(&self) -> StreamController<'_> {
        StreamController::new(self)
    }

    /// Returns `true` if authentication credentials have been set.
    pub fn auth_enabled(&self) -> bool {
        self.auth.is_some()
    }

    /// Returns a copy of the authentication credentials, if present.
    pub fn auth_credentials(&self) -> Option<(String, String)> {
        self.auth.clone()
    }

    /// Executes the command every command will use this
    pub(crate) async fn execute<C: MistCommand>(&self, command: C) -> Result<C::Response> {
        tracing::info!(command = C::NAME, "Executing Mist API command");
        self.transport.execute(command).await
    }
}

/// Builder for creating a [`MistClient`] with custom configuration.
///
/// The builder requires an HTTP client to be provided via [`with_client`] before
///
/// # Example
/// ```
/// use std::sync::Arc;
/// use reqwest::Client;
/// use mist_client::MistClientBuilder;
///
/// let client = Arc::new(Client::new());
/// let mist_client = MistClientBuilder::new("http://localhost:4242")
///     .with_client(client)
///     .with_auth("user", "pass")
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct MistClientBuilder {
    /// The base URL of the Mist API (e.g., `http://localhost:4242`).
    pub mist_api_url: String,
    /// The HTTP client used for all requests.
    pub client: Client,
    /// Optional authentication credentials (username, password).
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
    pub fn build(self) -> MistClient {
        let auth = self.auth;
        let client = self.client;

        let api = Arc::new(
            HttpTransportBuilder::new()
                .with_client(client.clone())
                .with_url(self.mist_api_url)
                .build(),
        );

        MistClient {
            auth,
            transport: api,
            auth_result: None,
        }
    }
}
