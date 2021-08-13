mod error;

use std::{path::PathBuf};

use fs_err as fs;

use crate::error::{CrateResult, Error};
use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Synchronizes a Directory's backup
struct CliArgs {
    #[argh(positional)]
    origin: PathBuf,

    #[argh(positional)]
    destination: PathBuf,
}

/// Parses and validates command-line arguments
fn validate_args() -> CrateResult<CliArgs> {
    let args: CliArgs = argh::from_env();
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
        },
        false => {
            // The destination directory does not exist,
            // so we must create it.
            fs::create_dir_all(&*args.destination)?;
            eprintln!("warn: '{}' was created.", args.destination.display());
        }
        _ => {}
    }


    Ok(args)
}

fn main() {
    // Doing this so a nicer error message gets printed out
    if let Err(err) = real_main() {
        eprintln!("Error: {}", err);
    }
}

fn real_main() -> CrateResult<()> {
    let args = validate_args()?;

    Ok(())
}
