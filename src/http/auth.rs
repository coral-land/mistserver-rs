use crate::{
    Result,
    commands::authorize::{AuthCredentials, AuthorizeCommand},
    http::MistApi,
};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AuthResponse {
    pub status: Option<AuthStatus>,
    pub challenge: Option<String>,
}

impl AuthResponse {
    pub fn needs_challenge(&self) -> bool {
        matches!(self.status, Some(AuthStatus::Chall))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthResponseWrapper {
    pub authorize: AuthResponse,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AuthStatus {
    Ok,
    Chall,
    NoAcc,
    AccMade,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AuthResult {
    NotRequired,
    Required(AuthResponse),
}

pub struct MistAuthController {
    auth: Option<(String, String)>,
    api: MistApi,
}

impl MistAuthController {
    pub(crate) fn new(
        client: Arc<Client>,
        mist_api_url: String,
        auth: Option<(String, String)>,
    ) -> Self {
        let (username, password) = auth.clone().unwrap_or_else(|| ("".into(), "".into()));

        Self {
            auth,
            api: MistApi::new(mist_api_url, client),
        }
    }

    pub fn auth_enabled(&self) -> bool {
        self.auth.is_some()
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
    pub async fn authorize(&self) -> Result<AuthResult> {
        let Some((username, password)) = &self.auth else {
            return Ok(AuthResult::NotRequired);
        };

        let auth_command = AuthorizeCommand {
            authorize: AuthCredentials {
                username: username.clone(),
                password: password.clone(),
            },
        };

        let response: AuthResponseWrapper = self.api.send(auth_command).await?;
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
    pub async fn authorize_with_challenge(&self, challenge: impl AsRef<str>) -> Result<AuthResult> {
        let (username, password) = self
            .auth
            .as_ref()
            .ok_or_else(|| crate::MistError::Auth("authentication is not configured".into()))?;

        let auth_hash = self.compute_auth_hash(password, challenge.as_ref());

        let auth_command = AuthorizeCommand {
            authorize: AuthCredentials {
                username: username.clone(),
                password: auth_hash,
            },
        };

        let response: AuthResponseWrapper = self.api.send(auth_command).await?;
        Ok(AuthResult::Required(response.authorize))
    }

    fn compute_auth_hash(&self, password: &str, challenge: &str) -> String {
        let password_hash = format!("{:x}", md5::compute(password.as_bytes()));
        let combined = format!("{password_hash}{challenge}");

        format!("{:x}", md5::compute(combined.as_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Result, utils::build_http_client};

    use mockito::{Matcher, Server};
    use std::{sync::Arc, time::Duration};

    fn test_client() -> Arc<Client> {
        Arc::new(build_http_client(Duration::from_secs(10)).expect("failed to build test client"))
    }

    #[test]
    fn auth_response_needs_challenge() {
        let response = AuthResponse {
            status: Some(AuthStatus::Chall),
            challenge: Some("abc".into()),
        };

        assert!(response.needs_challenge());
    }

    #[test]
    fn auth_response_does_not_need_challenge() {
        let response = AuthResponse {
            status: Some(AuthStatus::Ok),
            challenge: None,
        };

        assert!(!response.needs_challenge());
    }

    #[tokio::test]
    async fn authorize_returns_not_required_when_auth_disabled() -> Result<()> {
        let controller =
            MistAuthController::new(test_client(), "http://localhost:8080/api".into(), None);
        let result = controller.authorize().await?;

        assert_eq!(result, AuthResult::NotRequired);
        Ok(())
    }

    #[tokio::test]
    async fn authorize_returns_challenge_response() -> Result<()> {
        let mut server = Server::new_async().await;

        let response = serde_json::to_string(&AuthResponseWrapper {
            authorize: AuthResponse {
                status: Some(AuthStatus::Chall),
                challenge: Some("challenge_str".into()),
            },
        })?;

        let _mock = server
            .mock("GET", "/api")
            .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(response)
            .create_async()
            .await;

        let controller = MistAuthController::new(
            test_client(),
            format!("{}/api", server.url()),
            Some(("admin".into(), "password".into())),
        );

        let result = controller.authorize().await?;

        assert_eq!(
            result,
            AuthResult::Required(AuthResponse {
                status: Some(AuthStatus::Chall),
                challenge: Some("challenge_str".into()),
            })
        );

        Ok(())
    }

    #[tokio::test]
    async fn authorize_with_challenge_returns_ok() -> Result<()> {
        let mut server = Server::new_async().await;

        let response = serde_json::to_string(&AuthResponseWrapper {
            authorize: AuthResponse {
                status: Some(AuthStatus::Ok),
                challenge: None,
            },
        })?;

        let _mock = server
            .mock("GET", "/api")
            .match_query(Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(response)
            .create_async()
            .await;

        let controller = MistAuthController::new(
            test_client(),
            format!("{}/api", server.url()),
            Some(("admin".into(), "password".into())),
        );

        let result = controller.authorize_with_challenge("challenge_str").await?;

        assert_eq!(
            result,
            AuthResult::Required(AuthResponse {
                status: Some(AuthStatus::Ok),
                challenge: None,
            })
        );

        Ok(())
    }

    #[test]
    fn computes_expected_auth_hash() {
        let controller = MistAuthController::new(
            test_client(),
            "http://localhost".into(),
            Some(("admin".into(), "password".into())),
        );

        let hash = controller.compute_auth_hash("password", "challenge_str");
        let password_hash = format!("{:x}", md5::compute("password".as_bytes()));
        let expected = format!(
            "{:x}",
            md5::compute(format!("{password_hash}challenge_str").as_bytes())
        );

        assert_eq!(hash, expected);
    }
}
