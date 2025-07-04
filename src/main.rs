use clap::Parser;
use reopal::cli::{Args, Commands, Config};
use reopal::db;
use reopal::maintenance;
use reopal::scanner;
use reopal::web::{AppState, WebServer};
use rusqlite::{Connection, Result};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config_str = fs::read_to_string(&args.config)?;
    let config: Config = serde_yaml::from_str(&config_str)?;

    let conn = Connection::open(&config.db_path)?;
    db::init_db(&conn)?;

    match args.command {
        Commands::Import => {
            println!("Running import...");
            scanner::scan_directory(&config.directory, &conn)?;
            println!("Import complete.");
        }
        Commands::Maintenance => {
            println!("Running import before maintenance...");
            scanner::scan_directory(&config.directory, &conn)?;
            println!("Import complete. Running maintenance...");
            if let Some(maint_config) = config.maintenance {
                maintenance::run_maintenance(&conn, &maint_config.quota, maint_config.dry_run)?;
            } else {
                println!("Maintenance configuration not found in config file.");
            }
        }
        Commands::Web => {
            println!("Starting web viewer...");
            let default_config = Default::default();
            let web_config = config.web_viewer.as_ref().unwrap_or(&default_config);
            let host = web_config.host.clone();
            let port = web_config.port;
            let state = AppState::new(conn, config);
            let server = WebServer::new(state);
            server.start(&host, port).await?;
        }
    }

    Ok(())
}
