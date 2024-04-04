#[cfg(test)]
mod tests {
    use crate::format;
    use crate::logger;
    use serial_test::serial;

    const LOGLEVEL: &str = "Trace";
    const AUTHOR_1: &str = "Beckett, Simon";
    const AUTHOR_2: &str = "Cross, Ethan";
    const AUTHOR_3: &str = "King, Stephen";
    const FORMATTED_AUTHOR_1: &str = "Simon Beckett";
    const FORMATTED_AUTHOR_2: &str = "Ethan Cross";
    const FORMATTED_AUTHOR_3: &str = "Stephen King";
    const HTML_STRING_1: &str = "Bd. 7\nKnochenkälte / David Hunter Bd.7\nSimon Beckett\n0 Sterne\nBuch (Gebunden)\n26.00 €\nVorbestellen\nErscheint am 30.09.2024";
    const HTML_STRING_2: &str = "Im Labyrinth der Rache\nEthan Cross\n0 Sterne\nTaschenbuch\n13.00 €\nVorbestellen\nErscheint am 30.08.2024";
    const RELEASE_TITLE_1: &str = "Knochenkälte / David Hunter Bd.7";
    const RELEASE_TITLE_2: &str = "Im Labyrinth der Rache";
    const HTML_STRING_WRONG_FORMAT_TITLE: &str =
        "Simon Beckett\n0 Sterne\nBuch (Gebunden)\n26.00 €\nVorbestellen\nErscheint am 30.09.2024";
    const HTML_STRING_WRONG_FORMAT_DATE_1: &str = "Bd. 7\nKnochenkälte / David Hunter Bd.7\nSimon Beckett\n0 Sterne\nBuch (Gebunden)\n26.00 €\nVorbestellen\nErschienen am 30.09.2024";
    const HTML_STRING_WRONG_FORMAT_DATE_2: &str = "Bd. 7\nKnochenkälte / David Hunter Bd.7\nSimon Beckett\n0 Sterne\nBuch (Gebunden)\n26.00 €\nVorbestellen\nErscheint am 99.99.999999";

    #[test]
    #[serial]
    fn test_format_author_name() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            format::format_author_name(AUTHOR_1).unwrap(),
            FORMATTED_AUTHOR_1
        );
        assert_eq!(
            format::format_author_name(AUTHOR_2).unwrap(),
            FORMATTED_AUTHOR_2
        );
        assert_eq!(
            format::format_author_name(AUTHOR_3).unwrap(),
            FORMATTED_AUTHOR_3
        );
    }

    #[test]
    #[serial]
    fn test_format_release_title() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        assert_eq!(
            format::format_release_title(HTML_STRING_1, FORMATTED_AUTHOR_1).unwrap(),
            RELEASE_TITLE_1
        );
        assert_eq!(
            format::format_release_title(HTML_STRING_2, FORMATTED_AUTHOR_2).unwrap(),
            RELEASE_TITLE_2
        );
    }

    #[test]
    #[serial]
    fn test_format_release_date() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        // create release dates in correct format
        let naive_date = chrono::NaiveDate::from_ymd_opt(2024, 9, 30);
        let naive_datetime = chrono::NaiveDateTime::new(
            naive_date.unwrap(),
            chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );
        let release_date_1 =
            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(naive_datetime, chrono::Utc);

        let naive_date = chrono::NaiveDate::from_ymd_opt(2024, 8, 30);
        let naive_datetime = chrono::NaiveDateTime::new(
            naive_date.unwrap(),
            chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );
        let release_date_2 =
            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(naive_datetime, chrono::Utc);

        assert_eq!(
            format::format_release_date(HTML_STRING_1).unwrap(),
            release_date_1
        );
        assert_eq!(
            format::format_release_date(HTML_STRING_2).unwrap(),
            release_date_2
        );
    }

    #[test]
    #[serial]
    fn test_format_error_cases() {
        logger::init_logger(LOGLEVEL).expect("Could not initialize logger");

        // test author name in wrong format
        assert!(format::format_author_name("").is_err());

        // test html string in wrong format, so no title can be parsed
        assert!(
            format::format_release_title(HTML_STRING_WRONG_FORMAT_TITLE, FORMATTED_AUTHOR_1)
                .is_err()
        );

        // test author name in wrong format, so it can not be found in html string
        assert!(format::format_release_title(HTML_STRING_1, "").is_err());

        // test html string in wrong formats, so no release date can be parsed
        assert!(format::format_release_date(HTML_STRING_WRONG_FORMAT_DATE_1).is_err());
        assert!(format::format_release_date(HTML_STRING_WRONG_FORMAT_DATE_2).is_err());
    }
}
