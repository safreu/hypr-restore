mod tests;
use shared::{SNAPSHOT_PATH, DB_PATH};

fn main() -> std::io::Result<()>{
    snapshot::create_snapshot(DB_PATH, SNAPSHOT_PATH, true)
}
