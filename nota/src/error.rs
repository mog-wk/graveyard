pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to parse config at {0}")]
    ConfigParseError(usize),
    #[error("failed to get ")]
    PathError,
    #[error("")]
    IO(#[from] std::io::Error),
}
