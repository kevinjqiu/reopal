use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Import video files from a directory.
    Import(ImportArgs),
}

#[derive(Parser, Debug)]
pub struct ImportArgs {
    /// The path to the ReoLink video directory.
    #[arg(short, long)]
    pub directory: String,

    /// The path to the SQLite database file.
    #[arg(short = 'b', long, default_value = "reopal.db")]
    pub db_path: String,
}