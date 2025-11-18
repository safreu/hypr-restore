#[cfg(test)]
mod event_db_tests {
    use tempfile::tempdir;
    use shared::event_entry::EventEntry;
    use crate::event_db::EventDb;

    #[test]
    fn construct_new_event_db() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("tracker.db");
        let event_db: EventDb = EventDb::new(db_path.clone());

        assert!(event_db.table.is_empty());
    }

    #[test]
    fn insert_event() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("tracker.db");
        let mut event_db: EventDb = EventDb::new(db_path.clone());

        let event = EventEntry::new(
            &vec![
                "open_event".to_string(),
                "address".to_string(),
                "workspace".to_string(),
                "class".to_string(),
                "title".to_string()
            ]
        );

        event_db.insert(event.clone());
        let content = event_db.file_handler.read_file().unwrap();

        assert_eq!(event_db.table[0].to_string(), event.to_string());
        assert!(content.contains(event.to_string().as_str()));
    }

    #[test]
    fn remove_event() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("tracker.db");
        let mut event_db: EventDb = EventDb::new(db_path.clone());

        let event = EventEntry::new(
            &vec![
                "open_event".to_string(),
                "address".to_string(),
                "workspace".to_string(),
                "class".to_string(),
                "title".to_string()
            ]
        );

        event_db.insert(event.clone());

        let result = event_db.remove(event.address());

        let content = event_db.file_handler.read_file().unwrap();


        assert!(result.is_ok());
        assert!(!content.contains(event.to_string().as_str()));
        assert!(event_db.table.is_empty());
    }

    #[test]
    fn update_workspace_with_fitting_entry() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("tracker.db");
        let mut event_db: EventDb = EventDb::new(db_path.clone());

        let mut event = EventEntry::new(
            &vec![
                "open_event".to_string(),
                "address".to_string(),
                "workspace".to_string(),
                "class".to_string(),
                "title".to_string()
            ]
        );

        event_db.insert(event.clone());

        let result = event_db.update_workspace("address", "new_workspace");

        let content = event_db.file_handler.read_file().unwrap();

        let new_event = event.set_workspace("new_workspace");

        assert!(result.is_some());
        assert!(content.contains(new_event.to_string().as_str()));
        assert_eq!(event_db.table[0].to_string(), new_event.to_string());
    }

    #[test]
    fn update_workspace_with_fitting_entry_but_incorrect_address() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("tracker.db");
        let mut event_db: EventDb = EventDb::new(db_path.clone());

        let event = EventEntry::new(
            &vec![
                "open_event".to_string(),
                "address".to_string(),
                "workspace".to_string(),
                "class".to_string(),
                "title".to_string()
            ]
        );

        event_db.insert(event.clone());

        let result = event_db.update_workspace("not_existing", "new_workspace");

        let content = event_db.file_handler.read_file().unwrap();

        assert!(result.is_none());
        assert!(content.contains(event.to_string().as_str()));
        assert_eq!(event_db.table[0].to_string(), event.to_string());
    }

    #[test]
    fn update_workspace_without_fitting_entry() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("tracker.db");
        let mut event_db: EventDb = EventDb::new(db_path.clone());

        let event = EventEntry::new(
            &vec![
                "open_event".to_string(),
                "address".to_string(),
                "workspace".to_string(),
                "class".to_string(),
                "title".to_string()
            ]
        );

        event_db.insert(event.clone());

        let result = event_db.update_workspace("not_existing", "new_workspace");

        let content = event_db.file_handler.read_file().unwrap();

        assert!(result.is_none());
        assert!(content.contains(event.clone().to_string().as_str()));
        assert_eq!(event_db.table[0].to_string(), event.to_string());
    }

    #[test]
    fn get_index_with_fitting_entry() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("tracker.db");
        let mut event_db: EventDb = EventDb::new(db_path.clone());

        let event = EventEntry::new(
            &vec![
                "open_event".to_string(),
                "address".to_string(),
                "workspace".to_string(),
                "class".to_string(),
                "title".to_string()
            ]
        );

        event_db.insert(event);

        let result = event_db.get_index("address");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn get_index_without_fitting_entry() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("tracker.db");
        let mut event_db: EventDb = EventDb::new(db_path.clone());

        let event = EventEntry::new(
            &vec![
                "open_event".to_string(),
                "address".to_string(),
                "workspace".to_string(),
                "class".to_string(),
                "title".to_string()
            ]
        );

        event_db.insert(event);

        let result = event_db.get_index("not_existing");
        assert!(result.is_none());
    }
}