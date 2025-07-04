use crate::db;
use crate::models::VideoRecording;
use rayon::prelude::*;
use rusqlite::{Connection, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Scans the given directory in parallel, parses file information, and inserts it into the database.
pub fn scan_directory(dir_path: &str, conn: &Connection) -> Result<()> {
    let paths: Vec<PathBuf> = WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|p| p.is_file() && p.extension().map_or(false, |ext| ext == "mp4"))
        .collect();

    let records: Vec<VideoRecording> = paths
        .par_iter()
        .filter_map(|path| parse_path(path))
        .collect();

    let tx = conn.unchecked_transaction()?;
    for record in records {
        let changes = db::insert_record(&tx, &record)?;
        if changes > 0 {
            println!("Added: {}", record.file_path);
        }
    }
    tx.commit()?;

    Ok(())
}

/// Parses the file path to extract video metadata.
fn parse_path(path: &Path) -> Option<VideoRecording> {
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

    Some(VideoRecording {
        camera_name,
        date,
        start_time,
        end_time,
        file_path: file_path.to_string(),
        file_size,
        deleted: false,
    })
}
