use std::path::Path;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// invalid file format error
    #[error("Invalid formatting")]
    InvalidFormattingError,

    /// file system error
    #[error("Required file: {file} was not fould in system\nMust be in {default_location}")]
    RequiredFileNotFould {
        file: String,
        default_location: String,
    },
    /// generic IO error
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Debug: {0}")]
    Debug(String),

    #[error("CLI error")]
    CLI,

    #[error(transparent)]
    RegEx(#[from] regex::Error),
}

impl std::convert::From<String> for Error {
    fn from(value: String) -> Self {
        Self::Debug(value)
    }
}
