mod cli;
mod error;
mod file_utils;

use std::path::Path;

use file_utils::backup_folder;

use crate::error::CrateResult;

fn main() {

    let p = Path::new("/foo/bar/zee");
    dbg!(p.parent());

    if let Err(err) = smol::block_on(real_main()) {
        eprintln!("Error: {}", err);
    }
}

async fn real_main() -> CrateResult<()> {
    let args = cli::validate_args()?;

    backup_folder(&args).await
}
