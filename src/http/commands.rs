use serde::Serialize;

#[derive(Serialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthorizeCommand {
    pub authorize: Credentials,
}
