mod error;
mod cli;
mod file_utils;

use crate::file_utils::dir_walk;
use crate::error::{CrateResult};

fn main() {
    // Doing this so a nicer error message gets printed out
    if let Err(err) = real_main() {
        eprintln!("Error: {}", err);
    }
}



fn real_main() -> CrateResult<()> {
    let args = cli::validate_args()?;

    for entry in dir_walk(&*args.origin) {
        println!("Entry: {}", entry.display());
    }

    Ok(())
}
