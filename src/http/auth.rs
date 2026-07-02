//! Authentication controller for the Mist API.
//!
//! This module handles the authentication process for the Mist API,
//! including challenge-response authentication. It provides a controller
//! that can perform the initial authorization and complete challenge-based
//! authentication when required.

use crate::{
    MistClient, Result,
    commands::authorize::{AuthCredentials, AuthResponse, AuthorizeCommand},
};

use serde::{Deserialize, Serialize};

/// Authentication status codes returned by the Mist API.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AuthStatus {
    /// Authentication succeeded.
    Ok,
    /// Challenge required – further step needed.
    Chall,
    /// No account found.
    NoAcc,
    /// Account created (typically after successful registration).
    AccMade,
}

/// Result of an authentication attempt.
///
/// Indicates whether authentication is not required (disabled on the client)
/// or required with the server's response.
#[derive(Debug, Clone, PartialEq)]
pub enum AuthResult {
    /// Authentication is not configured on the client.
    NotRequired,
    /// Authentication is required and the server responded.
    Required(AuthResponse),
}

impl AuthResult {
    pub fn needs_challenge(&self) -> bool {
        match self {
            AuthResult::Required(AuthResponse { status, challenge }) => match status {
                Some(AuthStatus::Chall) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn challenge(&self) -> Option<String> {
        match self {
            AuthResult::Required(AuthResponse { status, challenge }) => match status {
                Some(AuthStatus::Chall) => challenge.clone(),
                _ => None,
            },
            _ => None,
        }
    }
}

/// Controller for handling Mist API authentication.
///
/// Manages the authentication flow, including initial authorization
/// and challenge response. It uses the underlying [`MistApi`] to send
/// commands and stores credentials only if authentication is enabled.
#[derive(Debug, Clone)]
pub struct AuthController<'a> {
    auth: Option<(String, String)>,
    client: &'a MistClient,
}

impl<'a> AuthController<'a> {
    /// Creates a new authentication controller.
    ///
    /// # Arguments
    /// * `client` - HTTP client to use for API calls.
    /// * `mist_api_url` - Base URL of the Mist API.
    /// * `auth` - Optional username/password pair. If `None`, authentication is disabled.
    pub(crate) fn new(auth: Option<(String, String)>, client: &'a MistClient) -> Self {
        Self { auth, client }
    }

    /// Returns `true` if authentication credentials are set.
    pub fn auth_enabled(&self) -> bool {
        self.auth.is_some()
    }

    /// Authorize with auto handling of challenge
    /// this is the only method you need to call.
    pub async fn authorize(&self) -> Result<()> {
        let auth_result = self.authorize_with_password().await?;

        match (auth_result.needs_challenge(), auth_result.challenge()) {
            (true, Some(challenge)) => {
                let auth_result = self.authorize_with_challenge(challenge).await?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Performs the first authentication step.
    ///
    /// If authentication is disabled on the client,
    /// [`AuthResult::NotRequired`] is returned.
    ///
    /// If authentication is enabled, the server response
    /// is returned inside [`AuthResult::Required`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mistserver_rs::MistClient;
    ///
    /// let client = MistClient::builder("http://localhost:4242/api")
    ///     .with_auth("admin", "password")
    ///     .build();
    ///
    /// let result = client.auth().authorize().await?;
    /// # Ok::<(), mistserver_rs::MistError>(())
    /// ```
    async fn authorize_with_password(&self) -> Result<AuthResult> {
        let Some((username, password)) = &self.auth else {
            return Ok(AuthResult::NotRequired);
        };

        let credentials = AuthCredentials {
            username: username.clone(),
            password: password.clone(),
        };

        let auth_command = AuthorizeCommand::new(credentials);
        let response = self.client.execute(auth_command).await?;

        Ok(AuthResult::Required(response.authorize))
    }

    /// Completes challenge-based authentication.
    ///
    /// Call this only when the previous response contains
    /// [`AuthStatus::Chall`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// let result = auth.authorize().await?;
    ///
    /// if let AuthResult::Required(response) = result {
    ///     if response.needs_challenge() {
    ///         auth.authorize_with_challenge(
    ///             response.challenge.unwrap()
    ///         ).await?;
    ///     }
    /// }
    /// ```
    async fn authorize_with_challenge(&self, challenge: impl AsRef<str>) -> Result<AuthResult> {
        let (username, password) = self.auth.as_ref().ok_or_else(|| {
            crate::MistError::Auth(
                "authentication is not configured but trying to perform auth challenge".into(),
            )
        })?;

        let auth_hash = self.compute_auth_hash(password, challenge.as_ref());
        let credentials = AuthCredentials {
            username: username.clone(),
            password: auth_hash,
        };

        let command = AuthorizeCommand::new(credentials);
        let response = self.client.execute(command).await?;

        Ok(AuthResult::Required(response.authorize))
    }

    /// Computes the MD5-based authentication hash required for challenge response.
    ///
    /// The algorithm is: MD5(password) concatenated with the challenge,
    /// then MD5 of that result.
    fn compute_auth_hash(&self, password: &str, challenge: &str) -> String {
        let password_hash = format!("{:x}", md5::compute(password.as_bytes()));
        let combined = format!("{password_hash}{challenge}");

        format!("{:x}", md5::compute(combined.as_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_result_not_required_has_no_challenge() {
        let result = AuthResult::NotRequired;

        assert!(!result.needs_challenge());
        assert_eq!(result.challenge(), None);
    }

    #[test]
    fn auth_result_chall_needs_challenge() {
        let result = AuthResult::Required(AuthResponse {
            status: Some(AuthStatus::Chall),
            challenge: Some("challenge".into()),
        });

        assert!(result.needs_challenge());
        assert_eq!(result.challenge(), Some("challenge".into()));
    }

    #[test]
    fn auth_result_ok_does_not_need_challenge() {
        let result = AuthResult::Required(AuthResponse {
            status: Some(AuthStatus::Ok),
            challenge: Some("challenge".into()),
        });

        assert!(!result.needs_challenge());
        assert_eq!(result.challenge(), None);
    }

    #[test]
    fn auth_result_without_status_does_not_need_challenge() {
        let result = AuthResult::Required(AuthResponse {
            status: None,
            challenge: Some("challenge".into()),
        });

        assert!(!result.needs_challenge());
        assert_eq!(result.challenge(), None);
    }

    #[test]
    fn compute_auth_hash_returns_expected_hash() {
        let unsafe_client = std::mem::MaybeUninit::zeroed();

        let controller = AuthController {
            auth: None,
            client: unsafe { unsafe_client.assume_init_ref() },
        };

        let hash = controller.compute_auth_hash("password", "challenge");

        let password_hash = format!("{:x}", md5::compute("password"));
        let expected = format!("{:x}", md5::compute(format!("{password_hash}challenge")));

        assert_eq!(hash, expected);
    }
}
