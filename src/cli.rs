use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the YAML configuration file.
    #[arg(short, long, default_value = "config.yml")]
    pub config: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Import video files from a directory.
    Import,
    /// Run maintenance to enforce disk quota.
    Maintenance,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub directory: String,
    pub db_path: String,
    pub maintenance: Option<MaintenanceConfig>,
}

#[derive(Debug, Deserialize)]
pub struct MaintenanceConfig {
    pub quota: String,
    #[serde(default)]
    pub dry_run: bool,
}