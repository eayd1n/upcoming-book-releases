mod authors;
mod cli;
mod customtypes;
mod format;
mod logger;
mod releases;
mod scraper;
mod tests;

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    // Capture the current time before executing the program
    let start_time = std::time::Instant::now();

    let args = cli::Args::parse();

    // first of all, set up the logger
    logger::init_logger(&args.loglevel.to_lowercase())?;

    log::debug!("{args:?}");

    // get the authors
    let authors = authors::read_authors(&args.authors_file)?;

    // parse the HTML contents to get the potential upcoming releases
    let upcoming_releases = scraper::parse_contents(authors).await?;

    // Create releases file
    releases::create_releases(upcoming_releases, &args.dest_release, &args.release_file)?;

    // Capture the current time after executing the program
    let end_time = std::time::Instant::now();

    // Calculate the elapsed time
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();

    log::info!("Elapsed time: {:.2} seconds", elapsed_time);

    Ok(())
}
