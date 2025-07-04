use clap::Parser;
use reopal::cli::{Args, Commands};
use reopal::db;
use reopal::scanner;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Import(import_args) => {
            let conn = Connection::open(&import_args.db_path)?;
            db::init_db(&conn)?;
            scanner::scan_directory(&import_args.directory, &conn)?;
            println!("Import complete.");
        }
    }

    Ok(())
}