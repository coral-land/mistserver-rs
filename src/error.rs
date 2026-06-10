pub type Result<T> = std::result::Result<T, MistError>;

#[derive(thiserror::Error, Debug)]
pub enum MistError {
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("MistServer API Error in endpoint {0}, error: {1}")]
    Api(String, String),

    #[error("MistServer Auth Error: {0}")]
    Auth(String),

    #[error("URL Parse Error: {0}")]
    UrlParseError(#[from] url::ParseError),
}
