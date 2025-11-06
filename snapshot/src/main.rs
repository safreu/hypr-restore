mod tests;

use std::fs;
use std::path::Path;
use shared::{SNAPSHOT_PATH, DB_PATH};
fn main() -> std::io::Result<()>{
    create_snapshot(SNAPSHOT_PATH, DB_PATH)
}

pub fn create_snapshot(db_path: &str, snapshot_path: &str) -> std::io::Result<()> {
    if Path::exists(Path::new(&db_path)) {
        fs::copy(db_path, snapshot_path)?;
        fs::remove_file(db_path)?
    }
    Ok(())
}
