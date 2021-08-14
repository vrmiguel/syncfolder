use std::path::Path;
use fs_err as fs;

use crate::error::CrateResult;

pub async fn copy_file(from: &Path, to: &Path) -> CrateResult<()> {
    let destination_parent = to.parent().unwrap_or(Path::new("/"));
    if !destination_parent.exists() {
        fs::create_dir_all(destination_parent)?;   
    }

    fs::copy(from, to)?;

    // TODO: set the backup's modified date to be equal to
    // the original file's modified date

    Ok(())
}