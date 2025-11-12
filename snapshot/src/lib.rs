use std::fs;
use std::path::Path;

pub fn create_snapshot(db_path: &str, snapshot_path: &str, delete: bool) -> std::io::Result<()> {
    if Path::exists(Path::new(&db_path)) {
        fs::copy(db_path, snapshot_path)?;
        if delete { fs::remove_file(db_path)? }
    }
    Ok(())
}
