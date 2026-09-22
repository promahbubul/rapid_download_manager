use thiserror::Error;

#[derive(Error, Debug)]
pub enum RapidError {
    #[error("HTTP network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Download cancelled or paused")]
    Cancelled,

    #[error("Server returned non-success status: {0}")]
    HttpStatus(reqwest::StatusCode),

    #[error("Server does not support range requests for multi-threading")]
    RangeNotSupported,

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Engine error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, RapidError>;
