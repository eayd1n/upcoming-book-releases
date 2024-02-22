use crate::customtypes::UpcomingRelease;
use crate::format;
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
                let formatted_author = match format::format_author_name(author) {
                    Ok(rearranged) => rearranged,
                    Err(err) => {
                        log::error!(
                            "Failed to get formatted author name for '{}': {}",
                            &author,
                            err
                        );
                        continue;
                    }
                };

                if formatted_content.contains(&formatted_author) {
                    let formatted_title =
                        match format::format_release_title(&formatted_content, &formatted_author) {
                            Ok(title) => title,
                            Err(err) => {
                                log::error!(
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
                            log::error!("Failed to get formatted date for '{}': {}", &author, err);
                            continue;
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

#[cfg(test)]
mod scraper_tests;
