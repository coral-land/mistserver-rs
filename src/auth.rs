use std::sync::Arc;

use crate::{
    Config, MistError, Result,
    http::commands::{AuthorizeCommand, Credentials},
};

use reqwest::Client;
use serde::Deserialize;
use url::Url;

/// Holds everything needed for authorization.
/// client: Reqwest Client
/// config: MistRs Config object
pub struct MistAuthController {
    client: Arc<Client>,
    config: Arc<Config>,
}

/// # Auth Response
/// This is based on what mist server will respond to your
/// authorization requests.
/// If everything goes as expected you will get the status
/// witch is a string and a challenge also a string.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AuthResponse {
    pub status: Option<AuthStatus>,
    pub challenge: Option<String>,
}

impl AuthResponse {
    pub fn needs_challenge(&self) -> bool {
        if let Some(status) = self.status.clone() {
            return status == AuthStatus::Chall;
        }
        false
    }
}

/// To parse the response from the mist api server.
#[derive(Debug, Clone, Deserialize)]
pub struct AuthResponseWrapper {
    pub authorize: AuthResponse,
}

/// Mist Server will return one of the statuses back.
/// current login status. Either "OK", "CHALL", "NOACC" or "ACC_MADE".
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AuthStatus {
    Ok,
    Chall,
    NoAcc,
    AccMade,
}

/// Auth Result when MistAUthController::authorize() is called
/// If Not required, you should bypass authorization process.
/// If required, then do the process :).
#[derive(Debug, Clone, PartialEq)]
pub enum AuthResult {
    NotRequired,
    Required(AuthResponse),
}

impl MistAuthController {
    pub fn new(client: Arc<Client>, config: Arc<Config>) -> Self {
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
    /// use mistserver_rs::{
    ///  AuthResult, MistAuthController, Result, config::Config, http::client::build_http_client,
    /// };
    /// use std::{sync::Arc, time::Duration};
    ///
    /// #[tokio::main]
    /// async fn main()-> Result<()> {
    ///     let client = Arc::new(build_http_client(Duration::from_secs(10))?);
    ///     let config = Arc::new(Config {
    ///         mist_url: "http://localhost:8080".into(),
    ///         auth: None,
    ///     });
    ///
    ///     let mac = MistAuthController::new(client.clone(), config.clone());
    ///     let auth_result: AuthResult = mac.authorize().await.unwrap();
    ///     match auth_result {
    ///         AuthResult::NotRequired => {}
    ///         AuthResult::Required(_) => panic!("Expected NotRequired, got Required"),
    ///     }
    ///
    ///     assert!(matches!(auth_result, AuthResult::NotRequired));
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn authorize(&self) -> Result<AuthResult> {
        if !self.config.auth_enabled() {
            return Ok(AuthResult::NotRequired);
        }

        let (username, password) = self.config.auth.as_ref().unwrap();

        let auth_command = AuthorizeCommand {
            authorize: Credentials {
                username: username.clone(),
                password: password.clone(),
            },
        };

        let auth_response = self.send_auth_request(auth_command).await?;

        match auth_response {
            AuthResponse {
                status: Some(AuthStatus::Chall),
                challenge,
            } => {
                if let Some(challenge) = challenge {
                    let auth_hash = self.compute_auth_hash(&password, &challenge);
                    let auth_command = AuthorizeCommand {
                        authorize: Credentials {
                            username: username.clone(),
                            password: auth_hash,
                        },
                    };
                    let auth_response = self.send_auth_request(auth_command).await?;
                    return Ok(AuthResult::Required(auth_response));
                } else {
                    return Err(MistError::Auth(
                        "Api Responded with challenge status but no challenge returned".into(),
                    ));
                }
            }

            _ => Ok(AuthResult::Required(auth_response)),
        }
    }

    /// Sends auth request to the mist api server and returns the auth response.
    /// This is a helper function for the authorize function.
    async fn send_auth_request(&self, auth_command: AuthorizeCommand) -> Result<AuthResponse> {
        let mut request_url = Url::parse(&self.config.mist_url)?;
        let json_auth_command = serde_json::to_string(&auth_command)?;
        request_url
            .query_pairs_mut()
            .append_pair("command", &json_auth_command);

        let response = self.client.get(request_url).send().await?;
        let auth_response: AuthResponseWrapper = response.json().await?;

        Ok(auth_response.authorize)
    }

    /// Computes the auth hash using the password and the challenge.
    /// This is based on the mist server's authentication mechanism.
    fn compute_auth_hash(&self, password: &str, challenge: &str) -> String {
        let password_hash_hex = format!("{:x}", md5::compute(password.as_bytes()));
        let combined = format!("{}{}", password_hash_hex, challenge);

        format!("{:x}", md5::compute(combined.as_bytes()))
    }
}
