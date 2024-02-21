/// Set up the logger. It can be only initialized once.
use anyhow::Result;
use env_logger::Env;

const CRATE_NAME: &str = "upcoming_book_releases";

/// Initialize the logger.
///
/// # Arguments
///
/// loglevel - The loglevel to use
///
/// # Return
///
/// Ok() - Successfully initialized logger
/// Err(err) - Some error occured
pub fn init_logger(loglevel: log::LevelFilter) -> Result<()> {
    static START: std::sync::Once = std::sync::Once::new();

    START.call_once(|| {
        env_logger::Builder::from_env(Env::default().default_filter_or("info"))
            .format_timestamp(None)
            .filter_module(CRATE_NAME, loglevel)
            .init();
    });

    Ok(())
}
