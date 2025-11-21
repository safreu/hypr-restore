use std::fs;
use std::path::{Path, PathBuf};
use crate::shared;

mod snapshot_tests;

pub fn execute() -> std::io::Result<()> {
    create_snapshot(&shared::db_path(), &shared::snapshot_path(), true)

}

/// Creates a snapshot on a given file
///
///  # Arguments
///  * `to_be_snapped` := Path to the file you want to snapshot
///  * `snapshot_path` := Path to the snapshot
///  * `delete` := Flag if you want to delete the original file, or not
///
///  # Returns
///  io Result if it was successfully or not
pub fn create_snapshot(to_be_snapped: &PathBuf, snapshot_path: &PathBuf, delete: bool) -> std::io::Result<()> {
    if Path::exists(Path::new(&to_be_snapped)) {
        fs::copy(to_be_snapped, snapshot_path)?;
        if delete { fs::remove_file(to_be_snapped)? }
    }
    Ok(())
}
