#[cfg(test)]
mod tests {
    use crate::authors;
    use crate::logger;
    use serial_test::serial;
    use std::io::Write;

    const AUTHORS_FILE: &str = "/tmp/authors";
    const NON_EXISTING_FILE: &str = "non-existing-file";
    const AUTHOR_1: &str = "Brown, Dan";
    const AUTHOR_2: &str = "Cross, Ethan";
    const AUTHOR_3: &str = "King, Stephen";
    const LOGLEVEL: &str = "Trace";

    #[test]
    #[serial]
    fn test_read_authors() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        // create authors file and write exemplary data into it
        let _ = std::fs::remove_file(AUTHORS_FILE);
        assert!(!std::path::Path::new(AUTHORS_FILE).exists());

        let mut authors_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(AUTHORS_FILE)
            .expect("Cannot open file");

        writeln!(authors_file, "{}", AUTHOR_1).expect("Writing of {} failed");
        writeln!(authors_file, "{}", AUTHOR_2).expect("Writing of {} failed");
        writeln!(authors_file, "{}", AUTHOR_3).expect("Writing of {} failed");
        assert!(std::path::Path::new(AUTHORS_FILE).exists());

        // now get the authors
        let authors = match authors::read_authors(AUTHORS_FILE) {
            Ok(extracted_authors) => extracted_authors,
            Err(_) => Vec::new(),
        };
        assert!(!authors.is_empty());

        let number_of_lines =
            linecount::count_lines(std::fs::File::open(AUTHORS_FILE).unwrap()).unwrap();

        assert_eq!(authors.len(), number_of_lines);

        // exemplary data to compare with
        let author_1 = AUTHOR_1.into();
        let author_2 = AUTHOR_2.into();
        let author_3 = AUTHOR_3.into();

        let authors_iter: Vec<_> = authors.into_iter().map(String::from).collect();

        assert!(authors_iter.contains(&author_1));
        assert!(authors_iter.contains(&author_2));
        assert!(authors_iter.contains(&author_3));

        // cleanup
        let _ = std::fs::remove_file(AUTHORS_FILE);
        assert!(!std::path::Path::new(AUTHORS_FILE).exists());
    }

    #[test]
    #[serial]
    fn test_authors_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        // test non-existing file
        let mut success: bool;
        match authors::read_authors(NON_EXISTING_FILE) {
            Ok(_) => success = true,
            Err(_) => success = false,
        }
        assert!(!success);

        // test existing file with no contents
        let _ = std::fs::File::create(AUTHORS_FILE).expect("Failed to create file");
        assert!(std::path::Path::new(AUTHORS_FILE).exists());

        match authors::read_authors(AUTHORS_FILE) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // cleanup
        let _ = std::fs::remove_file(AUTHORS_FILE);
        assert!(!std::path::Path::new(AUTHORS_FILE).exists());
    }
}
