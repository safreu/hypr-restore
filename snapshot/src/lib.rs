use std::fs;
use std::path::{Path, PathBuf};

pub fn create_snapshot(db_path: &PathBuf, snapshot_path: &PathBuf, delete: bool) -> std::io::Result<()> {
    if Path::exists(Path::new(&db_path)) {
        fs::copy(db_path, snapshot_path)?;
        if delete { fs::remove_file(db_path)? }
    }
    Ok(())
}
