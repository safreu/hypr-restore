use std::fs;
use std::path::Path;
use shared::{SNAPSHOT_PATH, DB_PATH};
fn main() -> std::io::Result<()>{
    if Path::exists(Path::new(&DB_PATH)) {
        fs::copy(DB_PATH, SNAPSHOT_PATH)?;
        fs::remove_file(DB_PATH)?
    }
    Ok(())
}
