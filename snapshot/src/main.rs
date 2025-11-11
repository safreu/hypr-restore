mod tests;
mod lib;

use shared::{SNAPSHOT_PATH, DB_PATH};
use crate::lib::create_snapshot;

fn main() -> std::io::Result<()>{
    create_snapshot(DB_PATH, SNAPSHOT_PATH, true)
}
