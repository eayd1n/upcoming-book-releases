//! This module scrapes HTML files from a webpage to get information about upcoming releases.

use crate::customtypes::UpcomingRelease;
use crate::format;
use anyhow::{Context, Result};

const WELTBILD_URL: &str = "https://www.weltbild.de";
const SEARCH: &str = "/suche/";
const RELEASE_YEAR: &str = "?jahr=0";
const TYPE: &str = "&node=%2Fbuecher";
const LANGUAGE: &str = "&sprache=%2Flanguage%2Fger";
static ONLY_BOOKS: [&str; 2] = ["Taschenbuch", "Buch"];

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
    let mut releasing_authors = std::collections::HashSet::new();

    // now get the data from Weltbild.de
    // WELTBILD_URL: The URL to Weltbild.de
    // SEARCH: Initiate a search
    // RELEASE_YEAR: We want the current year
    // TYPE: We want book, no audibles or something similar
    // LANGUAGE: We only want books in german language
    for (index, author) in authors.iter().enumerate() {
        log::info!(
            "Processing author '{}' ({}/{})",
            &author,
            &index + 1,
            &authors.len()
        );

        let updated_author = author.replace(", ", "+");
        let url =
            WELTBILD_URL.to_owned() + SEARCH + &updated_author + RELEASE_YEAR + TYPE + LANGUAGE;

        log::debug!("URL to check: '{}'", &url);

        // Send a GET request to the URL and retrieve the response
        let response = client
            .get(url.clone())
            .send()
            .await
            .with_context(|| format!("Failed to send HTTP GET request to '{}'", &url))?;

        // Check if the request was successful
        if response.status().is_success() {
            log::info!(
                "Request was successful! Parsing HTML contents for: '{}'",
                &author
            );

            // Read the response body as a string
            let html_content = response
                .text()
                .await
                .with_context(|| "Failed to get HTML content")?;

            // Parse the HTML content
            let document = scraper::Html::parse_document(&html_content);

            // Define a selector to find all <div class="inner-flex-container"> tag
            let div_selector = scraper::Selector::parse("div.inner-flex-container").unwrap();
            let matching_divs = document.select(&div_selector);

            // Iterate over three elements (there should be no more upcoming releases per author)
            for div_elem in matching_divs.take(3) {
                // remove trailing whitespaces and blank lines from string
                let raw_content = div_elem.text().collect::<String>();
                let formatted_content = raw_content
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(|line| line.trim())
                    .collect::<Vec<_>>()
                    .join("\n");

                log::trace!(
                    "Formatted HTML content for '{}':\n{:?}",
                    &author,
                    &formatted_content
                );

                // rearrange the author name to search in the formatted content
                // if the author is not found, no upcoming release is available. Continue the for
                // loop then
                let formatted_author = match format::format_author_name(author) {
                    Ok(rearranged) => rearranged,
                    Err(err) => {
                        log::trace!(
                            "Failed to get formatted author name for '{}': {}",
                            &author,
                            err
                        );
                        continue;
                    }
                };

                if formatted_content.contains(&formatted_author)
                    && ONLY_BOOKS
                        .iter()
                        .any(|&sub| formatted_content.contains(sub))
                {
                    let formatted_title =
                        match format::format_release_title(&formatted_content, &formatted_author) {
                            Ok(title) => title,
                            Err(err) => {
                                log::trace!(
                                    "Failed to get formatted release title for '{}': {}",
                                    &author,
                                    err
                                );
                                continue;
                            }
                        };

                    let formatted_date = match format::format_release_date(&formatted_content) {
                        Ok(date) => date,
                        Err(err) => {
                            log::trace!("Failed to get formatted date for '{}': {}", &author, err);
                            continue;
                        }
                    };

                    log::info!(
                        "Upcoming release '{}' for '{}' available!",
                        &formatted_title,
                        &formatted_author
                    );
                    let upcoming_release: UpcomingRelease = UpcomingRelease::create(
                        formatted_author.clone(),
                        formatted_title,
                        formatted_date,
                    );
                    upcoming_releases.push(upcoming_release);
                    releasing_authors.insert(formatted_author);
                }
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
        &releasing_authors.len(),
        &authors.len()
    );
    Ok(upcoming_releases)
}
