//! This module processes the upcoming releases for given authors and write them to a file.

use crate::customtypes::UpcomingRelease;
use anyhow::{Context, Result};
use chrono::Datelike;
use std::io::Write;

/// Write the releases to a destination file. All releases are sorted by date.
///
/// # Arguments
///
/// releases – The upcoming releases.
/// destination – The destination path for the releases file
/// file_name – The file name of the release list to be created
///
/// # Return
///
/// Ok() - Successfully created the releases file
/// Err(err) - Some error occured
pub fn create_releases(
    mut releases: Vec<UpcomingRelease>,
    destination: &str,
    file_name: &str,
) -> Result<()> {
    log::trace!("releases::create_releases()");

    // first of all, given Vector should not be empty
    if releases.is_empty() {
        anyhow::bail!("No releases existing!");
    }
    // also the destination path should exist. If no, create it
    std::fs::create_dir_all(destination)
        .with_context(|| format!("Failed to create destination dir '{}'", destination))?;

    log::debug!("Got {} releases to process", releases.len());

    // sort releases by date
    releases.sort_by(|a, b| a.date.cmp(&b.date));

    // now write the sorted data into a file
    let releases_path = destination.to_owned() + "/" + file_name;

    // remove maybe existing file first before creating a new one
    if std::path::Path::new(&releases_path).exists() {
        std::fs::remove_file(&releases_path).with_context(|| {
            format!(
                "Failed to remove already existing release file '{}'",
                &releases_path
            )
        })?;
    }

    let mut releases_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(releases_path.clone())
        .with_context(|| {
            format!(
                "Failed to create/append/open release file '{}'",
                &releases_path
            )
        })?;

    // set title of releases file
    writeln!(releases_file, "Upcoming Book Releases")?;

    // Collect upcoming releases under same date if happening
    let mut formatted_time_global = "".to_string();

    for release in &releases {
        if release.author.is_empty() {
            anyhow::bail!("No author given: {:?}", release);
        }
        if release.title.is_empty() {
            anyhow::bail!("No book title given: {:?}", release);
        }

        log::debug!("Release to process: {:#?}", release);

        // make date more human-readable (and print it in german date format)
        let formatted_time_local = format!(
            "{}. {} {}",
            release.date.day(),
            month_name_german(release.date.month()),
            release.date.year()
        );

        if formatted_time_global != formatted_time_local {
            formatted_time_global = formatted_time_local;

            writeln!(releases_file)?;
            writeln!(releases_file, "{}", &formatted_time_global).with_context(|| {
                format!(
                    "Failed to write date to release file '{:?}'",
                    &releases_path
                )
            })?;
            writeln!(releases_file, "-----------------------------------------------------------------------------------")?
        }

        writeln!(
            releases_file,
            "{} - \"{}\"",
            &release.author, &release.title
        )
        .with_context(|| {
            format!(
                "Failed to write author '{}' and title '{}' into release file '{}'",
                &release.author, &release.title, &releases_path
            )
        })?;
    }

    log::info!("Successfully created releases file '{}'", &releases_path);

    Ok(())
}

/// Map month number to german month name.
///
/// # Arguments
///
/// month - The month number
///
/// # Return
///
/// The matching german month name
fn month_name_german(month: u32) -> &'static str {
    match month {
        1 => "Januar",
        2 => "Februar",
        3 => "März",
        4 => "April",
        5 => "Mai",
        6 => "Juni",
        7 => "Juli",
        8 => "August",
        9 => "September",
        10 => "Oktober",
        11 => "November",
        12 => "Dezember",
        _ => "Unknown",
    }
}
