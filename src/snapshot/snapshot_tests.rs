#[cfg(test)]
mod tests {
    use crate::snapshot::create_snapshot;
    use std::fs::File;
    use std::io::{Read, Write};
    use tempfile::tempdir;

    #[test]
    fn create_snapshot_existing_path() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let snapshot_path = dir.path().join("test.snapshot");

        File::create(&db_path).unwrap();
        let _ = create_snapshot(&db_path, &snapshot_path, true);

        assert!(snapshot_path.exists());
    }

    #[test]
    fn create_snapshot_remove_db() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let snapshot_path = dir.path().join("test.snapshot");

        File::create(&db_path).unwrap();
        let _ = create_snapshot(&db_path, &snapshot_path, true);

        assert!(!db_path.exists());
    }

    #[test]
    fn create_snapshot_not_existing_path() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("not existing");
        let snapshot_path = dir.path().join("test.snapshot");

        let _ = create_snapshot(&db_path, &snapshot_path, true);

        assert!(!db_path.exists());
        assert!(!snapshot_path.exists());
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

        let _ = create_snapshot(&db_path, &snapshot_path, true);

        let mut contents = String::new();
        File::open(&snapshot_path)
            .expect("Failed to open snapshot file")
            .read_to_string(&mut contents)
            .expect("Failed to read snapshot");

        assert_eq!(contents.trim(), "test data");
    }
}
