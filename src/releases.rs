/// This module processes the upcoming releases for given authors and write them to a file.
use anyhow::Result;
use chrono::Datelike;
use std::io::Write;

/// Custom data type to collect the upcoming releases.
#[derive(Debug)]
pub struct UpcomingRelease {
    author: String,
    title: String,
    date: chrono::DateTime<chrono::Utc>,
}

impl UpcomingRelease {
    pub fn create(author: String, title: String, date: chrono::DateTime<chrono::Utc>) -> Self {
        UpcomingRelease {
            author,
            title,
            date,
        }
    }
}

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
    std::fs::create_dir_all(destination)?;

    log::debug!("Got {} releases to process", releases.len());

    // sort releases by date
    releases.sort_by(|a, b| a.date.cmp(&b.date));

    // now write the sorted data into a file
    let releases_path = destination.to_owned() + "/" + file_name;
    let mut releases_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(releases_path.clone())?;

    for release in &releases {
        if release.author.is_empty() {
            anyhow::bail!("No author given: {:?}", release);
        }
        if release.title.is_empty() {
            anyhow::bail!("No book title given: {:?}", release);
        }

        log::debug!("Release to process: {:#?}", release);

        // make date more human-readable (and print it in german date format)
        let formatted_time = format!(
            "{}. {} {}",
            release.date.day(),
            month_name_german(release.date.month()),
            release.date.year()
        );

        writeln!(releases_file, "{}", formatted_time)?;
        writeln!(
            releases_file,
            "------------------------------------------------------------------------"
        )?;
        writeln!(releases_file, "{} - {}", release.author, release.title)?;
        writeln!(releases_file, "")?;
    }

    log::info!("Successfully created releases file '{}'", releases_path);

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

#[cfg(test)]
mod releases_tests;
