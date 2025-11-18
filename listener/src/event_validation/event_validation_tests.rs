#[cfg(test)]
mod event_validation_tests {
    use tempfile::tempdir;
    use shared::event_entry::EventEntry;
    use crate::event_validation::EventValidation;

    #[test]
    fn construct_event_validation() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("tracker.db");
        let ignore_path = dir.path().join("classes.ignore");
        let executables_path = dir.path().join("executables.path");
        let _ = EventValidation::new(db_path, ignore_path, executables_path);
    }

    #[test]
    fn try_insert() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("tracker.db");
        let ignore_path = dir.path().join("classes.ignore");
        let executables_path = dir.path().join("executables.path");
        let mut validator = EventValidation::new(db_path.clone(), ignore_path, executables_path);

        let event = EventEntry::new(
            &vec![
                "open_event".to_string(),
                "address".to_string(),
                "workspace".to_string(),
                "class".to_string(),
                "title".to_string()
            ]
        );

        let result = validator.try_insert(event);

        assert!(result.is_ok());
        assert!(db_path.exists());
    }
}