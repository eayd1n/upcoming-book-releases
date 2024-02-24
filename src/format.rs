//! This module provides functions to format author name, title and release date for an upcoming
//! release.

use anyhow::Result;

/// Rearrange the author name from "<surname, firstname>" to "<firstname surname>".
/// Example: "Brown, Dan" is rearranged to "Dan Brown"
///
/// # Arguments
///
/// author - Author name to rearrange
///
/// # Return
///
/// Ok(String) - Rearranged author name
/// Err(err) - Some error occured
pub fn format_author_name(author: &str) -> Result<String> {
    log::trace!("format::format_author_name()");

    let mut parts = author.split(", ");

    if let (Some(last_name), Some(first_name)) = (parts.next(), parts.next()) {
        // Rearrange the components
        return Ok(format!("{} {}", first_name, last_name));
    }

    anyhow::bail!("Failed to rearrange author name for: '{}'", author);
}

/// Parse the title of the upcoming release from an HTML content.
///
/// # Arguments
///
/// html_content - HTML content to parse the title from
/// author - The author as target. The title is the element before the author in the vector
///
/// # Result
///
/// Ok(String) - The title of the upcoming release
/// Err(err) - Some error occured
pub fn format_release_title(html_content: &str, author: &str) -> Result<String> {
    log::trace!("format::format_title()");

    let html_content_vec: Vec<&str> = html_content
        .split('\n')
        .filter(|&s| !s.is_empty())
        .collect();

    if let Some(index) = html_content_vec.iter().position(|&s| s.contains(author)) {
        if index > 0 {
            let title = html_content_vec[index - 1];

            Ok(title.to_string())
        } else {
            anyhow::bail!("'{}' is the first element, no element before it.", author);
        }
    } else {
        anyhow::bail!("'{}' not found", author);
    }
}

/// Parse the release date of an upcoming release from an HTML content.
///
/// # Arguments
///
/// html_content - HTML content to parse the release date from
///
/// # Result
///
/// Ok() - The release date of the upcoming release
/// Err(err) - Some error occured
pub fn format_release_date(html_content: &str) -> Result<chrono::DateTime<chrono::Utc>> {
    log::trace!("format::format_release_date()");

    // Define the regular expression pattern to extract the date
    let re = regex::Regex::new(r"Erscheint am (.+)").unwrap();

    // Search for the date substring
    if let Some(captures) = re.captures(html_content) {
        // Extract the date part
        let date_str = captures.get(1).unwrap().as_str();

        // Parse the date string into a DateTime object
        if let Ok(parsed_date) = chrono::NaiveDate::parse_from_str(date_str, "%d.%m.%Y") {
            // Convert to DateTime<Utc>
            let datetime_utc = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                parsed_date.and_hms_opt(0, 0, 0).unwrap(),
                chrono::Utc,
            );

            log::debug!("Parsed date: {:?}", datetime_utc);
            Ok(datetime_utc)
        } else {
            anyhow::bail!("Failed to parse date");
        }
    } else {
        anyhow::bail!("Date substring not found");
    }
}
