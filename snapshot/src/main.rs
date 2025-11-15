mod tests;

fn main() -> std::io::Result<()>{
    snapshot::create_snapshot(&shared::db_path(), &shared::snapshot_path(), true)
}
