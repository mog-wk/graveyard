pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("$1")]
    Dev(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    ENV(#[from] dotenv::Error),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Dev(format!("{}", value))
    }
}
