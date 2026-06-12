//! Authentication command structures for the Mist API.
//!
//! This module defines the command and credential structures used
//! for authenticating with the Mist server.

use serde::{Deserialize, Serialize};

/// Command for authorizing against the Mist server.
///
/// Wraps the authentication credentials inside an `authorize` field
/// as expected by the Mist API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeCommand {
    /// The credentials to send for authorization.
    pub authorize: AuthCredentials,
}

/// User credentials for authentication.
///
/// Contains the username and password (or password hash when responding
/// to a challenge) required for the authorization process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCredentials {
    /// Username or account identifier.
    pub username: String,
    /// Password or challenge‑response hash.
    pub password: String,
}
