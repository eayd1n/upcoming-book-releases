use crate::releases::UpcomingRelease;
/// This module scrapes HTML files from a webpage to get information about upcoming releases.
use anyhow::Result;

const WELTBILD_URL: &str = "https://www.weltbild.de";
const SEARCH: &str = "/suche/";
const RELEASE_YEAR: &str = "?jahr=0";
const TYPE: &str = "&node=%2Fbuecher";
const LANGUAGE: &str = "&sprache=%2Flanguage%2Fger";

/// Navigate to respective Weltbild URL and parse html contents to get potential upcoming release
/// per author.
///
/// # Arguments
///
/// authors - A list of authors to get the potential upcoming releases
///
/// # Return
///
/// Ok(Vec<UpcomingRelease>) - A list of upcoming releases. Could be empty for no upcoming release
/// Err(err) - Some error occured
pub async fn parse_contents(authors: Vec<String>) -> Result<Vec<UpcomingRelease>> {
    log::trace!("scraper::parse_contents()");

    // first of all, check whether list of authors is empty or not
    if authors.is_empty() {
        anyhow::bail!("List contains no authors!");
    }
    log::info!("Number of authors to be processed: {}", &authors.len());

    // Create vector of upcoming releases
    let mut upcoming_releases: Vec<UpcomingRelease> = Vec::new();

    // Create reqwest client
    let client = reqwest::Client::new();

    // now get the data from Weltbild.de
    // WELTBILD_URL: The URL to Weltbild.de
    // SEARCH: Initiate a search
    // RELEASE_YEAR: We want the current year
    // TYPE: We want book, no audibles or something similar
    // LANGUAGE: We only want books in german language
    for author in &authors {
        log::info!("Processing author '{}'", &author);

        let updated_author = author.replace(", ", "+");
        let url =
            WELTBILD_URL.to_owned() + SEARCH + &updated_author + RELEASE_YEAR + TYPE + LANGUAGE;

        log::info!("URL to check: '{}'", &url);

        // Send a GET request to the URL and retrieve the response
        let response = client.get(url).send().await?;

        // Check if the request was successful
        if response.status().is_success() {
            log::info!(
                "Reqest was successful! Parsing HTML contents for: '{}'",
                &author
            );

            // Read the response body as a string
            let html_content = response.text().await?;

            // Parse the HTML content
            let document = scraper::Html::parse_document(&html_content);

            // Define a selector to find the first <div class="inner-flex-container"> tag
            let div_selector = scraper::Selector::parse("div.inner-flex-container").unwrap();

            if let Some(div_elem) = document.select(&div_selector).next() {
                // remove trailing whitespaces and blank lines from string
                let raw_content = div_elem.text().collect::<String>();
                let formatted_content = raw_content
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(|line| line.trim())
                    .collect::<Vec<_>>()
                    .join("\n");

                log::debug!(
                    "Formatted HTML content for '{}':\n{:?}",
                    &author,
                    &formatted_content
                );

                // rearrange the author name to search in the formatted content
                // if the author is not found, no upcoming release is available. Continue the for
                // loop then
                let formatted_author = match format_author_name(author) {
                    Ok(rearranged) => rearranged,
                    Err(err) => {
                        log::error!("Failed to get formatted author name for '{}': {}", &author, err);
                        continue
                    }
                };

                if formatted_content.contains(&formatted_author) {
                    let formatted_title =
                        match format_release_title(&formatted_content, &formatted_author) {
                            Ok(title) => title,
                            Err(err) => {
                            log::error!("Failed to get formatted release title for '{}': {}", &author, err);
                            continue
                        }
                        };

                    let formatted_date = match format_release_date(&formatted_content) {
                        Ok(date) => date,
                        Err(err) =>  {
                            log::error!("Failed to get formatted date for '{}': {}", &author, err);
                            continue
                        }
                    };

                    let upcoming_release: UpcomingRelease =
                        UpcomingRelease::create(formatted_author, formatted_title, formatted_date);
                    upcoming_releases.push(upcoming_release);
                } else {
                    log::warn!("No upcoming release for '{}' available.", &formatted_author);
                    continue;
                }
            } else {
                anyhow::bail!("HTML parsing was not successful for: '{}'", &author);
            }
        } else {
            anyhow::bail!(
                "Request for author '{}' failed with status code: {:?}",
                &author,
                response.status()
            );
        }

        // wait one second before doing the next request
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    log::info!(
        "Upcoming releases found for {}/{} authors",
        &upcoming_releases.len(),
        &authors.len()
    );
    Ok(upcoming_releases)
}

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
fn format_author_name(author: &str) -> Result<String> {
    log::trace!("scraper::format_author_name()");

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
fn format_release_title(html_content: &str, author: &str) -> Result<String> {
    log::trace!("scraper::format_title()");

    let html_content_vec: Vec<&str> = html_content
        .split('\n')
        .filter(|&s| !s.is_empty())
        .collect();

    if let Some(index) = html_content_vec.iter().position(|&s| s == author) {
        if index > 0 {
            let title = html_content_vec[index - 1];

            log::info!(
                "Author '{}' has the following upcoming release: '{}'",
                &author,
                &title
            );
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
fn format_release_date(html_content: &str) -> Result<chrono::DateTime<chrono::Utc>> {
    log::trace!("scraper::format_release_date()");

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

            log::info!("Parsed date: {:?}", datetime_utc);
            Ok(datetime_utc)
        } else {
            anyhow::bail!("Failed to parse date");
        }
    } else {
        anyhow::bail!("Date substring not found");
    }
}

#[cfg(test)]
mod scraper_tests;
