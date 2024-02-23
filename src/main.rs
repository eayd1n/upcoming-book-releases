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
    let loglevel = match args.loglevel.to_lowercase().as_str() {
        "off" => log::LevelFilter::Off,
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Debug,
    };

    logger::init_logger(loglevel)?;

    log::debug!("{args:?}");

    // sort authors in given file if requested
    if let Some(filename) = args.sort_authors {
        log::info!("Try to sort the authors first");
        authors::sort_authors(&filename)?;
    }

    // get the authors
    let authors = authors::read_authors(&args.authors_file)?;

    // parse the HTML contents to get the potential upcoming releases
    let upcoming_releases = scraper::parse_contents(authors).await?;

    // Create releases file
    releases::create_releases(upcoming_releases, "/tmp", "releases")?;

    Ok(())
}
