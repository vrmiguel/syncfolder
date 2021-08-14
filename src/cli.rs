use std::path::PathBuf;

use argh::FromArgs;

use fs_err as fs;

use crate::error::{CrateResult, Error};

#[derive(FromArgs, PartialEq, Debug)]
/// Synchronizes a Directory's backup
pub struct CliArgs {
    #[argh(positional)]
    pub origin: PathBuf,

    #[argh(positional)]
    pub destination: PathBuf,
}

/// Parses and validates command-line arguments
pub fn validate_args() -> CrateResult<CliArgs> {
    let mut args: CliArgs = argh::from_env();
    if !args.origin.exists() {
        return Err(Error::OriginDoesNotExist(args.origin));
    }
    if !args.origin.is_dir() {
        return Err(Error::OriginIsNotADirectory(args.origin));
    }

    match args.destination.exists() {
        true if !args.destination.is_dir() => {
            // Supplied destination exists but is not a directory
            return Err(Error::DestinationIsNotADirectory(args.destination));
        }
        false => {
            // The destination directory does not exist,
            // so we must create it.
            fs::create_dir_all(&*args.destination)?;
            eprintln!("warn: '{}' was created.", args.destination.display());
        }
        _ => {}
    }

    args.origin = fs::canonicalize(args.origin)?;
    args.destination = fs::canonicalize(args.destination)?;

    Ok(args)
}
