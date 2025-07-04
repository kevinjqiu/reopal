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
    /// Start the web viewer server.
    Web,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub directory: String,
    pub db_path: String,
    pub maintenance: Option<MaintenanceConfig>,
    pub web_viewer: Option<WebViewerConfig>,
}

#[derive(Debug, Deserialize)]
pub struct MaintenanceConfig {
    pub quota: String,
    #[serde(default)]
    pub dry_run: bool,
}

#[derive(Debug, Deserialize)]
pub struct WebViewerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub auth: Option<AuthConfig>,
    pub thumbnails: Option<ThumbnailConfig>,
    #[serde(default)]
    pub max_concurrent_streams: u32,
    #[serde(default)]
    pub cache_size: String,
    pub features: Option<FeatureConfig>,
}

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_session_timeout")]
    pub session_timeout: String,
    #[serde(default)]
    pub admin_users: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ThumbnailConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_cache_dir")]
    pub cache_dir: String,
    #[serde(default = "default_quality")]
    pub quality: String,
}

#[derive(Debug, Deserialize)]
pub struct FeatureConfig {
    #[serde(default)]
    pub real_time_updates: bool,
    #[serde(default)]
    pub bulk_operations: bool,
    #[serde(default)]
    pub mobile_optimized: bool,
}

// Default value functions
fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_session_timeout() -> String {
    "24h".to_string()
}

fn default_cache_dir() -> String {
    "./thumbnails".to_string()
}

fn default_quality() -> String {
    "medium".to_string()
}

impl Default for WebViewerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            auth: None,
            thumbnails: None,
            max_concurrent_streams: 10,
            cache_size: "1GB".to_string(),
            features: None,
        }
    }
}
