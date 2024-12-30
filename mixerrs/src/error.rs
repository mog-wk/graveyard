#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to retrive music directory {0}")]
    MusicDirectoryNotFound(String),
    #[error("Currupted invalid directory")]
    MusicDirectoryCurrupted,
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

