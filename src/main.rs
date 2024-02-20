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
    let _authors = authors::read_authors(AUTHORS_FILE)?;

    // TODO: parse the HTML files
    // htmlparser::extract_upcoming_releases(HTML_DESTINATION)?;

    // TODO: Create releases file

    Ok(())
}
