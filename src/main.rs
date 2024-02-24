mod authors;
mod cli;
mod customtypes;
mod format;
mod logger;
mod releases;
mod scraper;
mod tests;

use anyhow::{Context, Result};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    // Capture the current time before executing the program
    let start_time = std::time::Instant::now();

    let args = cli::Args::parse();

    // first of all, set up the logger
    logger::init_logger(&args.loglevel)
        .with_context(|| format!("Failed to set loglevel '{}'", &args.loglevel))?;

    log::debug!("{args:?}");

    // get the authors
    let authors = authors::read_authors(&args.authors_file)
        .with_context(|| format!("Failed to extract authors from '{}'", &args.authors_file))?;

    // parse the HTML contents to get the potential upcoming releases
    let upcoming_releases = scraper::parse_contents(authors)
        .await
        .with_context(|| "Failed to scrape the HTML contents from Webpage")?;

    // Create releases file
    releases::create_releases(upcoming_releases, &args.dest_release, &args.release_file)
        .with_context(|| {
            format!(
                "Failed to create release file '{}', located in '{}'",
                &args.release_file, &args.dest_release
            )
        })?;

    // Capture the current time after executing the program
    let end_time = std::time::Instant::now();

    // Calculate the elapsed time
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();

    log::info!("Elapsed time: {:.2} seconds", elapsed_time);

    Ok(())
}
