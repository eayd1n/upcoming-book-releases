mod authors;
mod logger;
mod releases;
mod scraper;

use anyhow::Result;

const AUTHORS_FILE: &str =
    "/home/eaydin/Dokumente/Zeugs/rustprojects/upcoming_book_releases/authors";

#[tokio::main]
async fn main() -> Result<()> {
    // first of all, set up the logger
    logger::init_logger(log::LevelFilter::Debug)?;

    // get the authors
    let authors = authors::read_authors(AUTHORS_FILE)?;

    // parse the HTML contents to get the potential upcoming releases
    let upcoming_releases = scraper::parse_contents(authors).await?;

    // Create releases file
    releases::create_releases(upcoming_releases, "/tmp", "releases")?;

    Ok(())
}
