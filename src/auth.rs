use std::sync::Arc;

use crate::{Config, Result};
use reqwest::Client;

/// Holds everything needed for authorization.
/// client: Reqwest Client
/// config: MistRs Config object
pub struct MistAuthController {
    client: Client,
    config: Arc<Config>,
}

/// # Auth Response
/// This is based on what mist server will respond to your
/// authorization requests.
/// If everything goes as expected you will get the status
/// witch is a string and a challenge also a string.
#[derive(Debug, Clone)]
pub struct AuthResponse {
    pub status: Option<AuthStatus>,
    pub challenge: Option<String>,
}

/// Mist Server will return one of the statuses back.
/// current login status. Either "OK", "CHALL", "NOACC" or "ACC_MADE".

#[derive(Debug, Clone)]
pub enum AuthStatus {
    Ok,
    NoAccount,
    AccountMade,
    Challenge,
}

/// Auth Result when MistAUthController::authorize() is called
/// If Not required, you should bypass authorization process.
/// If required, then do the process :).
pub enum AuthResult {
    NotRequired,
    Required(AuthResponse),
}

impl MistAuthController {
    pub fn new(client: Client, config: Arc<Config>) -> Self {
        Self { client, config }
    }

    /// # Authorize
    /// In case authorization is not enabled, you will get:
    /// AuthResult::NotRequired
    /// and if it's enabled you will get:
    /// AuthResult::Required(AuthResponse)
    /// This should bypass the mechanisms for verifying challenges
    ///
    /// ### Example:
    ///
    /// ```rust
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = build_http_client();
    ///     let config = Arc::new(config());
    ///
    ///     let mac = MistAuthController::new(client.clone(), config.clone());
    ///     let auth_result: AuthResult = mac.authorize().await?;
    ///
    ///     match auth_result {
    ///         AuthResult::NotRequired => {}
    ///         AuthResult::Required(auth_response) => {}
    ///     }
    /// }
    /// ```
    ///
    pub async fn authorize(&self) -> Result<AuthResult> {
        if !self.config.auth_enabled() {
            return Ok(AuthResult::NotRequired);
        }

        let base_url = self.config.mist_url.clone();
        let api_url = format!("{base_url}/api");

        let response = self.client.get(api_url).send().await?;

        todo!()
    }
}
