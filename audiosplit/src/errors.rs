use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Cli(CliError),
    #[error(transparent)]
    CliFrom(#[from] CliError),
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("DEV ERROR")]
    DEV,
}

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("{:?}", 1)]
    InvalidPath(PathBuf),

    #[error("ext: {:?}", 1)]
    InvalidExtention(String),

    #[error("path: {:?}", 1)]
    NoExtention(PathBuf),

    #[error("{}: {:?}", num, line)]
    TimestampFormatError { num: usize, line: String },
}

#[derive(Debug, thiserror::Error)]
/// byte stream error
pub enum StreamError {
    #[error("{:?}", 1)]
    Gen(String),
}
