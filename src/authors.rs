/// This module provides functions to interact with a list containing authors.
use anyhow::Result;
use std::io::{BufRead, Write};

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

/// Insert a list of authors into a file. The file contents will be sorted afterwards.
///
/// # Arguments
///
/// authors - List of authors
/// filename - File which contains the (sorted) authors in the end
///
/// # Return
///
/// Ok() - Successfully inserted authors
/// Err(err) - Some error occured
pub fn insert_authors(authors: Vec<String>, filename: &str) -> Result<()> {
    log::trace!("authors::insert_authors()");

    // check emptiness of author's list
    if authors.is_empty() {
        anyhow::bail!("No authors passed to insert!");
    }

    // now start to insert the authors
    let mut authors_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    for author in &authors {
        authors_file.write_all(author.as_bytes())?;
        authors_file.write_all(b"\n")?;
        log::debug!("Wrote '{}' into '{}'", author, filename);
    }

    // sort the author's list
    sort_authors(filename)?;

    log::info!(
        "Successfully {} new authors were added into '{}'",
        authors.len(),
        filename
    );

    Ok(())
}

/// Sort the authors in the respective file.
///
/// # Arguments
///
/// filename - The file which needs a sort of authors
///
/// # Return
///
/// Ok() - Successfully sorted the authors
/// Err(err) - Some error occured
fn sort_authors(filename: &str) -> Result<()> {
    log::trace!("authors::sort_authors()");

    // open file, read lines and then sort them
    let authors_file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(authors_file);

    let mut authors: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    authors.sort();

    // open file again to write sorted data
    let mut authors_file = std::fs::File::create(filename)?;

    for author in &authors {
        authors_file.write_all(author.as_bytes())?;
        authors_file.write_all(b"\n")?;
    }

    log::info!("Current number of authors: {}", authors.len());

    Ok(())
}

#[cfg(test)]
mod authors_tests;
