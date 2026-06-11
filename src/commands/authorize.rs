use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeCommand {
    pub authorize: AuthCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCredentials {
    pub username: String,
    pub password: String,
}
