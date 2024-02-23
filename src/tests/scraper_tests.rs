#[cfg(test)]
mod tests {
    use crate::authors;
    use crate::logger;
    use crate::scraper;
    use serial_test::serial;

    const AUTHORS_FILE: &str = "src/tests/test_authors";
    const LOGLEVEL: &str = "Trace";

    #[tokio::test]
    #[serial]
    async fn test_parse_contents() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        // create vector of authors
        let authors_file = std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string()
            + "/"
            + AUTHORS_FILE;
        let authors = authors::read_authors(&authors_file);

        let success: bool;
        match scraper::parse_contents(authors.unwrap()).await {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(success);
    }

    #[tokio::test]
    #[serial]
    async fn test_scraper_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        let success: bool;

        // test empty list of authors
        let empty_authors: Vec<String> = Vec::new();
        match scraper::parse_contents(empty_authors).await {
            Ok(_) => success = true,
            Err(_) => success = false,
        };
        assert!(!success);
    }
}
