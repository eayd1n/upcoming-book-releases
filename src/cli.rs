//! This module contains the arguments passed via command line.

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the file containing your authors
    #[arg(short, long, default_value = "/home/authors")]
    pub authors_file: String,

    /// Path to the resulting release file
    #[arg(short, long, default_value = "/home/upcoming-releases")]
    pub release_file: String,

    /// Log level (off, warn, error, info, debug, trace)
    #[arg(short, long, default_value = "debug")]
    pub loglevel: String,
}
