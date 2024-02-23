#[cfg(test)]
mod tests {
    use crate::logger;
    use crate::scraper;
    use serial_test::serial;

    const LOGLEVEL: &str = "Trace";
    const AUTHOR_1: &str = "Cross, Ethan";
    const AUTHOR_2: &str = "Beckett, Simon";
    const AUTHOR_3: &str = "Brown, Dan";

    #[tokio::test]
    #[serial]
    async fn test_parse_contents() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        // create vector of authors
        let authors: Vec<String> = vec![
            AUTHOR_1.to_string(),
            AUTHOR_2.to_string(),
            AUTHOR_3.to_string(),
        ];

        let _ = scraper::parse_contents(authors).await;
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
