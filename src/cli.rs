use clap::Parser;

/// Program for running SQL queries against a CSV.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the CSV file.
    #[clap(short, long, value_parser)]
    pub file: Option<String>,

    /// SQL query to run.
    #[clap(value_parser)]
    pub query: String,
}
