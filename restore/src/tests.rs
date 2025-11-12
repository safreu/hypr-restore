#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::fs::File;
    use crate::ExecutablesHandler;
    use tempfile::tempdir;


    #[test]
    fn new_path_tacker() {
        let dir = tempdir().unwrap();
        let executables_path = dir.path().join("executables.path");
        let path_tracker = ExecutablesHandler::new(executables_path.to_str().unwrap());
        assert!(path_tracker.table.is_empty());
    }

    #[test]
    fn init_path_tacker_with_empty_file() {
        let dir = tempdir().unwrap();
        let executables_path = dir.path().join("executables.path");
        let mut path_tracker = ExecutablesHandler::new(executables_path.to_str().unwrap());

        path_tracker.init();

        assert!(path_tracker.table.is_empty());
        assert!(executables_path.exists());
    }

    #[test]
    fn init_path_tacker_with_not_empty_file() {
        let dir = tempdir().unwrap();
        let executables_path = dir.path().join("executables.path");
        let mut path_tracker = ExecutablesHandler::new(executables_path.to_str().unwrap());

        {
            let mut file = File::create(&executables_path).expect("Failed to create executables file");
            writeln!(file, "test, data").expect("Failed to write to executables");
        }

        path_tracker.init();
        assert!(!path_tracker.table.is_empty());
        assert_eq!(path_tracker.table.get("test"), Some(&"data".to_string()));
    }

    #[test]
    fn insert_new_entry_with_existing_file() {
        let dir = tempdir().unwrap();
        let executables_path = dir.path().join("executables.path");
        let mut path_tracker = ExecutablesHandler::new(executables_path.to_str().unwrap());
        {
            File::create(&executables_path).expect("Failed to create executables file");
        }

        path_tracker.init();
        path_tracker.insert(String::from("test"), String::from("data")).expect("Failed to write");
        let file_content = path_tracker.file_handler.read_file().expect("Failed to read file");

        assert_eq!(path_tracker.table.get("test"), Some(&String::from("data")));
        assert_eq!(file_content.get("test,data"), Some(&String::from("test,data")));
    }

    #[test]
    fn insert_new_entry_with_not_existing_file() {
        let dir = tempdir().unwrap();
        let executables_path = dir.path().join("executables.path");
        let mut path_tracker = ExecutablesHandler::new(executables_path.to_str().unwrap());

        path_tracker.init();
        path_tracker.insert(String::from("test"), String::from("data")).expect("Failed to write");
        let file_content = path_tracker.file_handler.read_file().expect("Failed to read file");

        assert_eq!(path_tracker.table.get("test"), Some(&String::from("data")));
        assert_eq!(file_content.get("test,data"), Some(&String::from("test,data")));
    }

    #[test]
    fn insert_changed_entry_with_existing_file() {
        let dir = tempdir().unwrap();
        let executables_path = dir.path().join("executables.path");
        let mut path_tracker = ExecutablesHandler::new(executables_path.to_str().unwrap());

        path_tracker.init();
        path_tracker.insert(String::from("test"), String::from("data")).expect("Failed to write");
        let file_content = path_tracker.file_handler.read_file().expect("Failed to read file");

        assert_eq!(path_tracker.table.get("test"), Some(&String::from("data")));
        assert_eq!(file_content.get("test,data"), Some(&String::from("test,data")));
    }

    #[test]
    fn insert_new_value_with_existing_file() {
        let dir = tempdir().unwrap();
        let executables_path = dir.path().join("executables.path");
        let mut path_tracker = ExecutablesHandler::new(executables_path.to_str().unwrap());

        {
            File::create(&executables_path).expect("Failed to create executables file");
        }

        path_tracker.init();
        path_tracker.insert(String::from("test"), String::from("data")).expect("Failed to write");
        let mut file_content = path_tracker.file_handler.read_file().expect("Failed to read file");

        assert_eq!(path_tracker.table.get("test"), Some(&String::from("data")));
        assert_eq!(file_content.get("test,data"), Some(&String::from("test,data")));

        path_tracker.insert(String::from("test"), String::from("changed_data")).expect("Failed to write");

        file_content = path_tracker.file_handler.read_file().expect("Failed to read file");

        assert_eq!(path_tracker.table.get("test"), Some(&String::from("changed_data")));
        assert_eq!(file_content.get("test,changed_data"), Some(&String::from("test,changed_data")));
    }

    #[test]
    fn insert_new_entry_with_existing_file_still_containing_other_entry() {
        let dir = tempdir().unwrap();
        let executables_path = dir.path().join("executables.path");
        let mut path_tracker = ExecutablesHandler::new(executables_path.to_str().unwrap());

        {
            File::create(&executables_path).expect("Failed to create executables file");
        }

        path_tracker.init();
        path_tracker.insert(String::from("test"), String::from("data")).expect("Failed to write");
        path_tracker.insert(String::from("different_test"), String::from("different_data")).expect("Failed to write");

        let file_content = path_tracker.file_handler.read_file().expect("Failed to read file");

        assert_eq!(path_tracker.table.get("test"), Some(&String::from("data")));
        assert_eq!(file_content.get("test,data"), Some(&String::from("test,data")));
        assert_eq!(path_tracker.table.get("different_test"), Some(&String::from("different_data")));
        assert_eq!(file_content.get("different_test,different_data"), Some(&String::from("different_test,different_data")));
    }

    #[test]
    fn get_executable_entry_with_existing_file() {
        let dir = tempdir().unwrap();
        let executables_path = dir.path().join("executables.path");
        let mut path_tracker = ExecutablesHandler::new(executables_path.to_str().unwrap());

        {
            File::create(&executables_path).expect("Failed to create executables file");
        }

        path_tracker.init();
        path_tracker.insert(String::from("test"), String::from("data")).expect("Failed to write");
        path_tracker.insert(String::from("different_test"), String::from("different_data")).expect("Failed to write");


        assert_eq!(path_tracker.get_executable_entry("test"), Some(String::from("data")));
        assert_eq!(path_tracker.get_executable_entry("different_test"), Some(String::from("different_data")));
        assert_eq!(path_tracker.get_executable_entry("different_test2"), None);
    }

}