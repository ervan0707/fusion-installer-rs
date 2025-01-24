use thiserror::Error;

#[derive(Error, Debug)]
pub enum FusionError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Version error: {0}")]
    Version(String),

    #[error("Build number not found")]
    BuildNotFound,

    #[error("System error: {0}")]
    System(String),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
}

pub type Result<T> = std::result::Result<T, FusionError>;
