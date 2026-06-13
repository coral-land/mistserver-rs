pub type Result<T> = std::result::Result<T, MistError>;

#[derive(thiserror::Error, Debug)]
pub enum MistError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("API error on {endpoint}: {message}")]
    Api { endpoint: String, message: String },

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("URL parse error: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Timeout waiting for response")]
    Timeout,
}

pub trait ErrorContext<T> {
    fn context(self, msg: impl Into<String>) -> Result<T>;
}
