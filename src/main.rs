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
    let args = cli::Args::parse();

    // first of all, set up the logger
    logger::init_logger(&args.loglevel.to_lowercase())?;

    log::debug!("{args:?}");

    // get the authors
    let authors = authors::read_authors(&args.authors_file)?;

    // parse the HTML contents to get the potential upcoming releases
    let upcoming_releases = scraper::parse_contents(authors).await?;

    // Create releases file
    releases::create_releases(upcoming_releases, "/tmp", "releases")?;

    Ok(())
}
