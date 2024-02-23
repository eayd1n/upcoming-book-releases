#[cfg(test)]
mod tests {
    use crate::customtypes::UpcomingRelease;
    use crate::logger;
    use crate::releases;
    use serial_test::serial;
    use std::io::{BufRead, BufReader};

    const DEST: &str = "/tmp";
    const FILE_NAME: &str = "releases";
    const RELEASE_FILE: &str = "/tmp/releases";
    const LOGLEVEL: log::LevelFilter = log::LevelFilter::Trace;

    const AUTHOR_1: &str = "Brown, Dan";
    const TITLE_1: &str = "Sakrileg";

    const AUTHOR_2: &str = "King, Stephen";
    const TITLE_2: &str = "Shining";

    const AUTHOR_3: &str = "Rowling, J.K.";
    const TITLE_3: &str = "Harry Potter and the Prisoner of Askaban";

    #[test]
    #[serial]
    fn test_release_file_creation() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let _ = std::fs::remove_file(RELEASE_FILE);
        assert!(!std::path::Path::new(RELEASE_FILE).exists());

        // create some test data
        let releases: Vec<UpcomingRelease> = vec![
            UpcomingRelease::create(
                AUTHOR_1.to_string(),
                TITLE_1.to_string(),
                chrono::Utc::now()
                    .checked_add_signed(chrono::Duration::days(3))
                    .unwrap(),
            ),
            UpcomingRelease::create(
                AUTHOR_2.to_string(),
                TITLE_2.to_string(),
                chrono::Utc::now(),
            ),
            UpcomingRelease::create(
                AUTHOR_3.to_string(),
                TITLE_3.to_string(),
                chrono::Utc::now()
                    .checked_sub_signed(chrono::Duration::days(2))
                    .unwrap(),
            ),
        ];

        let success: bool;
        match releases::create_releases(releases, DEST, FILE_NAME) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(success);
        assert!(std::path::Path::new(RELEASE_FILE).exists());

        let mut author_1_line_number: usize = 0;
        let mut author_2_line_number: usize = 0;
        let mut author_3_line_number: usize = 0;

        // now open the file to check whether it was correctly sorted or not
        if let Ok(release_file) = std::fs::File::open(RELEASE_FILE) {
            // create a buffered reader to read the file line by line
            let reader = BufReader::new(release_file);

            // Iterate over the lines and search for the pattern
            for (line_number, line) in reader.lines().enumerate() {
                if let Ok(line_content) = line {
                    if line_content.contains(AUTHOR_1) {
                        author_1_line_number = line_number + 1;
                    }
                    if line_content.contains(AUTHOR_2) {
                        author_2_line_number = line_number + 1;
                    }
                    if line_content.contains(AUTHOR_3) {
                        author_3_line_number = line_number + 1;
                    }
                }
            }
        }

        assert_ne!(!author_1_line_number, 0);
        assert_ne!(!author_2_line_number, 0);
        assert_ne!(!author_3_line_number, 0);

        assert!(author_1_line_number > author_2_line_number);
        assert!(author_1_line_number > author_3_line_number);
        assert!(author_2_line_number > author_3_line_number);

        // cleanup
        let _ = std::fs::remove_file(RELEASE_FILE);
        assert!(!std::path::Path::new(RELEASE_FILE).exists());
    }

    #[test]
    #[serial]
    fn test_releases_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let mut success: bool;

        // pass empty vector
        let empty_vector = Vec::new();
        match releases::create_releases(empty_vector, DEST, FILE_NAME) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // pass incomplete data
        let missing_author: Vec<UpcomingRelease> = vec![UpcomingRelease::create(
            "".to_string(),
            TITLE_3.to_string(),
            chrono::Utc::now(),
        )];

        match releases::create_releases(missing_author, DEST, FILE_NAME) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        let missing_title: Vec<UpcomingRelease> = vec![UpcomingRelease::create(
            AUTHOR_2.to_string(),
            "".to_string(),
            chrono::Utc::now(),
        )];

        match releases::create_releases(missing_title, DEST, FILE_NAME) {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);

        // cleanup
        let _ = std::fs::remove_file(RELEASE_FILE);
        assert!(!std::path::Path::new(RELEASE_FILE).exists());
    }
}
