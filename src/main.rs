mod error;
mod cli;
mod file_utils;

use cli::CliArgs;
use file_utils::backup_folder;

use crate::file_utils::dir_walk;
use crate::error::{CrateResult};

fn main() {
    // Doing this so a nicer error message gets printed out
    if let Err(err) = smol::block_on(real_main()) {
        eprintln!("Error: {}", err);
    }
}

async fn real_main() -> CrateResult<()> {
    let args = cli::validate_args()?;

    backup_folder(&args).await
}
