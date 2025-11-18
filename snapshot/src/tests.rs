#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn create_snapshot_existing_path() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let snapshot_path = dir.path().join("test.snapshot");

        File::create(&db_path).unwrap();
        let _ = crate::create_snapshot(&db_path, &snapshot_path, true);

        assert_eq!(snapshot_path.exists(), true);
    }

    #[test]
    fn create_snapshot_remove_db() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let snapshot_path = dir.path().join("test.snapshot");

        File::create(&db_path).unwrap();
        let _ = crate::create_snapshot(&db_path, &snapshot_path, true);

        assert_eq!(db_path.exists(), false);
    }

    #[test]
    fn create_snapshot_not_existing_path() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("not existing");
        let snapshot_path = dir.path().join("test.snapshot");

        let _ = crate::create_snapshot(&db_path, &snapshot_path, true);

        assert_eq!(db_path.exists(), false);
        assert_eq!(snapshot_path.exists(), false);
    }

    #[test]
    fn db_content_gets_copied() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let snapshot_path = dir.path().join("test.snapshot");

        {
            let mut file = File::create(&db_path).expect("Failed to create db file");
            writeln!(file, "test data").expect("Failed to write to db");
        }

        let _ = crate::create_snapshot(&db_path, &snapshot_path, true);

        let mut contents = String::new();
        File::open(&snapshot_path)
            .expect("Failed to open snapshot file")
            .read_to_string(&mut contents)
            .expect("Failed to read snapshot");

        assert_eq!(contents.trim(), "test data");
    }

}