#[cfg(test)]
mod ignored_handler_tests {
    use std::fs;
    use tempfile::tempdir;
    use crate::ignored_handler::IgnoredHandler;

    #[test]
    fn construct_with_empty_file() {
        let dir = tempdir().unwrap();
        let ignore_path = dir.path().join("classes.ignore");
        let ignored_handler = IgnoredHandler::new(ignore_path);

        assert!(ignored_handler.ignored_classes.is_empty());
    }

    #[test]
    fn construct_with_content_in_file() {
        let dir = tempdir().unwrap();
        let ignore_path = dir.path().join("classes.ignore");

        fs::write(&ignore_path, "content").unwrap();

        let ignored_handler = IgnoredHandler::new(ignore_path.clone());

        assert!(ignored_handler.ignored_classes.contains("content"));
        assert!(fs::read_to_string(&ignore_path).unwrap().contains("content"));
    }

    #[test]
    fn should_not_skip_class_not_contained() {
        let dir = tempdir().unwrap();
        let ignore_path = dir.path().join("classes.ignore");

        fs::write(&ignore_path, "skippable_class").unwrap();

        let ignored_handler = IgnoredHandler::new(ignore_path.clone());

        let result =ignored_handler.should_ignore("unskippable_class", "address");

        assert_eq!(result, false);
    }

    #[test]
    fn should_skip_class_is_contained() {
        let dir = tempdir().unwrap();
        let ignore_path = dir.path().join("classes.ignore");

        fs::write(&ignore_path, "skippable_class").unwrap();

        let ignored_handler = IgnoredHandler::new(ignore_path.clone());

        let result =ignored_handler.should_ignore("skippable_class", "address");

        assert_eq!(result, true);
    }

    //Not sure how I should implement the tests for should_skip
    //TODO: Implement the tests for should_skip()
}