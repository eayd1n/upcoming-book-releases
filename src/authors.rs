//! This module provides functions to interact with a list containing authors.

use anyhow::Result;

/// Get the authors from a list. In general, they are listed as <surname, forename>.
///
/// # Arguments
///
/// filename - The file which contains the authors
///
/// # Return
///
/// Ok(Vec<String>) - If successful, return a list of authors
/// Err(err) - Some error occured
pub fn read_authors(filename: &str) -> Result<Vec<String>> {
    log::trace!("authors::read_authors()");

    // first of all, check whether file exists or not
    if !std::path::Path::new(filename).exists() {
        anyhow::bail!("File '{}' not found!", filename);
    }

    // now read the file line by line
    let mut authors = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        authors.push(line.to_string());
        log::debug!("Extracted '{}' from list", line)
    }

    if authors.is_empty() {
        anyhow::bail!("No author in '{}' found!", filename);
    }

    log::debug!("Extracted {} authors from the list", authors.len());

    Ok(authors)
}
