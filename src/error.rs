use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("origin directory (`{0}`) does not exist")]
    OriginDoesNotExist(PathBuf),
    #[error("`{0}` is not a directory")]
    OriginIsNotADirectory(PathBuf),
    #[error("`{0}` is not a directory")]
    DestinationIsNotADirectory(PathBuf),
    #[error("IO: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("`{0}`")]
    Walkdir(#[from] walkdir::Error)
}

// For some reason argh::FromArg starts to fail when
// this type is named Result, so we'll just name it CrateResult
pub type CrateResult<T> = std::result::Result<T, Error>;
