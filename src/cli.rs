use clap::Parser;

/// A CLI tool to process ReoLink video files and store them in a database.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The path to the ReoLink video directory.
    #[arg(short, long)]
    pub directory: String,

    /// The path to the SQLite database file.
    #[arg(short = 'b', long, default_value = "reopal.db")]
    pub db_path: String,
}
