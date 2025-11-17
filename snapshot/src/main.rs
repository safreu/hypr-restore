mod tests;

/// Runs the create_snapshot function to snapshot the db_file, with the delete flag set on true
fn main() -> std::io::Result<()>{
    snapshot::create_snapshot(&shared::db_path(), &shared::snapshot_path(), true)
}
