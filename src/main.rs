mod cli;
mod db;
mod models;
mod scanner;

use clap::Parser;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let conn = Connection::open(&args.db_path)?;

    db::init_db(&conn)?;
    scanner::scan_directory(&args.directory, &conn)?;

    println!("Database updated successfully.");
    Ok(())
}
