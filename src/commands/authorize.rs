use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AuthorizeCommand {
    pub authorize: AuthCredentials,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthCredentials {
    pub username: String,
    pub password: String,
}
