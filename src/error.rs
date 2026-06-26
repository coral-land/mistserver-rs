pub type Result<T> = std::result::Result<T, MistError>;

#[derive(thiserror::Error, Debug)]
pub enum MistError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("API error: {message}")]
    Api { message: String },

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("URL parse error: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error(
        "invalid stream name '{0}': only letters, numbers, underscores ('_'), and hyphens ('-') are allowed"
    )]
    InvalidStreamName(String),

    #[error("Timeout waiting for response")]
    Timeout,

    #[error("Stream Name Too Long, current: {0}, acceptable: <= 100 characters")]
    StreamNameTooLong(usize),

    #[error("Stream Invalid source: {0}")]
    InvalidSource(String),

    #[error("Invalid Push Url: {0}")]
    InvalidPushUrl(String),

    #[error("Invalid Host: {0}")]
    InvalidHost(String),

    #[error("Invalid Port: {0}")]
    InvalidPort(u16),

    #[error("Invalid Configuration: {0}")]
    InvalidConfiguration(String),
}

pub trait ErrorContext<T> {
    fn context(self, msg: impl Into<String>) -> Result<T>;
}
