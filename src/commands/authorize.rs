//! Authentication command structures for the Mist API.
//!
//! This module defines the command and credential structures used
//! for authenticating with the Mist server.

use serde::{Deserialize, Serialize};

use crate::{commands::traits::MistCommand, controllers::AuthStatus};

/// Command for authorizing against the Mist server.
///
/// Wraps the authentication credentials inside an `authorize` field
/// as expected by the Mist API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthorizeCommand {
    /// The credentials to send for authorization.
    pub authorize: AuthCredentials,
}

impl AuthorizeCommand {
    /// Creates a new Authorize command with given credentials
    ///
    /// # Arguments
    /// * `credentials` - a predefined `AuthCredentials`.
    pub fn new(credentials: AuthCredentials) -> Self {
        Self {
            authorize: credentials,
        }
    }
}

impl MistCommand for AuthorizeCommand {
    type Response = AuthResponseWrapper;
    const NAME: &'static str = "authorize";
}

/// Response from an authentication request.
///
/// Contains the authentication status and an optional challenge string
/// that must be used for completing the authentication if the status is `Chall`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AuthResponse {
    /// Status of the authentication attempt.
    pub status: Option<AuthStatus>,
    /// Challenge string for completing authentication, if required.
    pub challenge: Option<String>,
}

impl AuthResponse {
    /// Returns `true` if the response indicates a challenge is required.
    pub fn needs_challenge(&self) -> bool {
        matches!(self.status, Some(AuthStatus::Chall))
    }
}

/// Wrapper for the authentication response as returned by the Mist API.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthResponseWrapper {
    /// The actual authentication response inside the `authorize` field.
    pub authorize: AuthResponse,
}

/// User credentials for authentication.
///
/// Contains the username and password (or password hash when responding
/// to a challenge) required for the authorization process.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthCredentials {
    /// Username or account identifier.
    pub username: String,
    /// Password or challenge‑response hash.
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::traits::MistCommand;
    use serde_json::json;

    fn credentials() -> AuthCredentials {
        AuthCredentials {
            username: "admin".to_string(),
            password: "secret".to_string(),
        }
    }

    #[test]
    fn authorize_command_new() {
        let credentials = credentials();

        let command = AuthorizeCommand::new(credentials.clone());

        assert_eq!(command.authorize.username, credentials.username);
        assert_eq!(command.authorize.password, credentials.password);
    }

    #[test]
    fn authorize_command_serialization() {
        let command = AuthorizeCommand::new(credentials());

        let expected = json!({
            "authorize": {
                "username": "admin",
                "password": "secret"
            }
        });

        let actual = serde_json::to_value(command).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn authorize_command_deserialization() {
        let json = json!({
            "authorize": {
                "username": "admin",
                "password": "secret"
            }
        });

        let command: AuthorizeCommand = serde_json::from_value(json).unwrap();

        assert_eq!(command.authorize.username, "admin");
        assert_eq!(command.authorize.password, "secret");
    }

    #[test]
    fn auth_credentials_serialization() {
        let credentials = credentials();

        let expected = json!({
            "username": "admin",
            "password": "secret"
        });

        let actual = serde_json::to_value(credentials).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn auth_credentials_deserialization() {
        let json = json!({
            "username": "admin",
            "password": "secret"
        });

        let credentials: AuthCredentials = serde_json::from_value(json).unwrap();

        assert_eq!(credentials.username, "admin");
        assert_eq!(credentials.password, "secret");
    }

    #[test]
    fn authorize_command_name() {
        assert_eq!(AuthorizeCommand::NAME, "authorize");
    }
}
