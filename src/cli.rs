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
    /// Run maintenance to enforce disk quota.
    Maintenance(MaintenanceArgs),
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

#[derive(Parser, Debug)]
pub struct MaintenanceArgs {
    /// The path to the ReoLink video directory to scan before maintenance.
    #[arg(short, long)]
    pub directory: String,

    /// The disk space quota (e.g., "10GB", "500MB").
    #[arg(short, long)]
    pub quota: String,

    /// The path to the SQLite database file.
    #[arg(short = 'b', long, default_value = "reopal.db")]
    pub db_path: String,

    /// If set, the command will only print the actions to be taken.
    #[arg(long)]
    pub dry_run: bool,
}