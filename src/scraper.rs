/// This module scrapes HTML files from webpage to get information about upcoming releases.

use anyhow::Result;

const WELTBILD_URL: &str = "https://www.weltbild.de";
const SEARCH: &str = "/suche/";
const RELEASE_YEAR: &str = "?jahr=0";
const TYPE: &str = "&node=%2Fbuecher";
const LANGUAGE: &str = "&sprache=%2Flanguage%2Fger";

pub fn parse_contents(authors: Vec<String>) -> Result<()> {
    log::trace!("scraper::parse_contents()");

    // first of all, check whether list of authors is empty or not
    if authors.is_empty() {
        anyhow::bail!("List contains no authors!");
    }
    log::info!("Number of authors to be processed: {}", &authors.len());

    // create an headless_chrome browser
    let browser = headless_chrome::Browser::default()?;

    // now get the data from Weltbild.de
    // WELTBILD_URL: The URL to Weltbild.de
    // SEARCH: Initiate a search
    // RELEASE_YEAR: We want the current year
    // TYPE: We want book, no audibles or something similar
    // LANGUAGE: We only want books in german language
    for author in &authors {
        log::info!("Processing author '{}'", &author);

        let updated_author = author.replace(", ", "+");
        let url = WELTBILD_URL.to_owned() + SEARCH + &updated_author + RELEASE_YEAR + TYPE + LANGUAGE;

        log::info!("URL to check: '{}'", &url);
        
        // now navigate with headless_chrome to the respective page and try to get the information
        // from HTML
        let tab = browser.new_tab()?;
        tab.navigate_to(&url)?;

        // Wait for page to load completely
        let body = tab.wait_for_element("body")?;

        // now select the first <div class="inner-flex-container"> element
        let div_element = body.find_element("div.inner-flex-container a")?;

        // we want the "href" attribute to navigate to the new page
        let href_attribute = match div_element.get_attribute_value("href") {
            Ok(attr) => attr,
            Err(err) => anyhow::bail!("No 'href' attribute found: {}", err)
        };

        if let Some(href_url) = href_attribute {
            let new_url = WELTBILD_URL.to_owned() + &href_url;

            log::info!("Navigating to new URL: '{}'", &new_url);
            tab.navigate_to(&new_url)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod scraper_tests;
