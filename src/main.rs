use clap::Parser;
use rayon::prelude::*;
use rusqlite::{Connection, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// A CLI tool to process ReoLink video files and store them in a database.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the ReoLink video directory.
    #[arg(short, long)]
    directory: String,

    /// The path to the SQLite database file.
    #[arg(short = 'b', long, default_value = "reopal.db")]
    db_path: String,
}

#[derive(Debug)]
struct VideoRecord {
    camera_name: String,
    date: String,
    start_time: String,
    end_time: String,
    file_path: String,
    file_size: u64,
    deleted: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let conn = Connection::open(&args.db_path)?;

    init_db(&conn)?;
    scan_directory(&args.directory, &conn)?;

    println!("Database updated successfully.");
    Ok(())
}

/// Initializes the database and creates the 'videos' table if it doesn't exist.
fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS videos (
            file_path TEXT PRIMARY KEY,
            camera_name TEXT NOT NULL,
            date TEXT NOT NULL,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL,
            file_size INTEGER NOT NULL,
            deleted BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(())
}

/// Scans the given directory in parallel, parses file information, and inserts it into the database.
fn scan_directory(dir_path: &str, conn: &Connection) -> Result<()> {
    let paths: Vec<PathBuf> = WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|p| p.is_file() && p.extension().map_or(false, |ext| ext == "mp4"))
        .collect();

    let records: Vec<VideoRecord> = paths
        .par_iter()
        .filter_map(|path| parse_path(path))
        .collect();

    let tx = conn.unchecked_transaction()?;
    for record in records {
        let changes = insert_record(&tx, &record)?;
        if changes > 0 {
            println!("Added: {}", record.file_path);
        }
    }
    tx.commit()?;

    Ok(())
}

/// Parses the file path to extract video metadata.
fn parse_path(path: &Path) -> Option<VideoRecord> {
    let file_name = path.file_stem()?.to_str()?;
    let file_path = path.to_str()?;
    let parent_dir = path.parent()?.file_name()?.to_str()?;

    // Validate directory name format (MMDDYYYY)
    if parent_dir.len() != 8 || !parent_dir.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let date = parent_dir.to_string();

    // Parse filename format: <camera name>-00-<start time>-<end time>
    let parts: Vec<&str> = file_name.split('-').collect();
    if parts.len() != 4 {
        return None;
    }

    let camera_name = parts[0].to_string();
    let start_time = parts[2].to_string();
    let end_time = parts[3].to_string();

    if start_time.len() != 6 || end_time.len() != 6 {
        return None;
    }

    let metadata = std::fs::metadata(path).ok()?;
    let file_size = metadata.len();

    Some(VideoRecord {
        camera_name,
        date,
        start_time,
        end_time,
        file_path: file_path.to_string(),
        file_size,
        deleted: false,
    })
}

/// Inserts a single VideoRecord into the database, ignoring duplicates.
fn insert_record(conn: &Connection, record: &VideoRecord) -> Result<usize> {
    conn.execute(
        "INSERT OR IGNORE INTO videos (file_path, camera_name, date, start_time, end_time, file_size, deleted)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            &record.file_path,
            &record.camera_name,
            &record.date,
            &record.start_time,
            &record.end_time,
            &record.file_size,
            &record.deleted,
        ),
    )
}