use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("origin Directory (`{0}`) does not exist")]
    OriginDoesNotExist(PathBuf),
    #[error("`{0}` is not a Directory")]
    OriginIsNotADirectory(PathBuf),
    #[error("`{0}` is not a Directory")]
    DestinationIsNotADirectory(PathBuf),
    #[error("IO: `{0}`")]
    Io(#[from] std::io::Error)
}

// For some reason argh::FromArg starts to fail when
// this type is named Result, so we'll just name it CrateResult
pub type CrateResult<T> = std::result::Result<T, Error>;
