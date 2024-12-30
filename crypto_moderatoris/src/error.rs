pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    //#[error("")]
    //DEV,
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    ENV(#[from] dotenv::Error),

    #[error(transparent)]
    REQWEST(#[from] reqwest::Error),
    #[error("invalid status code $1")]
    RequestStatusCodeError(String),
}
