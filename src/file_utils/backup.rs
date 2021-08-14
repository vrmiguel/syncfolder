use fs_err as fs;
use std::path::Path;

use crate::{cli::CliArgs, error::CrateResult};

use super::{copy_file, dir_walk};

fn backup_is_updated(original: &Path, backup: &Path) -> CrateResult<bool> {
    // TODO: add a better check later on
    Ok(original.metadata()?.modified()? == backup.metadata()?.modified()?)
}

async fn backup_file(absolute_path: &Path, relative_path: &Path, dest: &Path) -> CrateResult<()> {
    // The same file name, but on the destination directory
    let file_in_dest = dest.join(relative_path);

    if file_in_dest.exists() {
        // There is a file in the destination folder with this name.
        // We must now check if this backup is updated
        if backup_is_updated(absolute_path, &*file_in_dest)? {
            // The backup is updated, so it's all good!
            return Ok(());
        }
    }

    // We must create (or update) the backup
    copy_file(absolute_path, &*&file_in_dest).await?;

    Ok(())
}

/// Backs up the folder given by args.origin into args.destination
pub async fn backup_folder(args: &CliArgs) -> CrateResult<()> {
    let base = &*args.origin;
    let dest = &*args.destination;

    for path in dir_walk(base) {
        // Stripping away the base folder from the path
        let path_relative = match path.strip_prefix(base) {
            Ok(path) => path,
            // TODO: treat this accordingly
            Err(_) => continue,
        };

        if let Err(err) = backup_file(&*path, path_relative, dest).await {
            eprintln!("{}", err);
        }
    }

    Ok(())
}
