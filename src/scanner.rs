use crate::db;
use crate::models::VideoRecording;
use chrono::{DateTime, TimeZone, Utc};
use rayon::prelude::*;
use rusqlite::{Connection, Result};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
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
    let filename_start_time = parts[2];
    let filename_end_time = parts[3];

    if filename_start_time.len() != 6 || filename_end_time.len() != 6 {
        return None;
    }

    // Calculate duration from filename times
    let start_seconds = parse_time_to_seconds(filename_start_time)?;
    let end_seconds = parse_time_to_seconds(filename_end_time)?;
    let duration_seconds = if end_seconds >= start_seconds {
        end_seconds - start_seconds
    } else {
        // Handle case where end time is past midnight (next day)
        (24 * 3600 - start_seconds) + end_seconds
    };

    let metadata = std::fs::metadata(path).ok()?;
    let file_size = metadata.len();

    // Use file creation time as end_time
    let created_time = metadata.created().ok()?;
    let end_time: DateTime<Utc> = created_time.into();

    // Calculate start_time as end_time minus duration
    let start_time = end_time - chrono::Duration::seconds(duration_seconds as i64);

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

/// Parse time string in HHMMSS format to seconds since midnight
fn parse_time_to_seconds(time_str: &str) -> Option<u32> {
    if time_str.len() != 6 {
        return None;
    }

    let hours: u32 = time_str[0..2].parse().ok()?;
    let minutes: u32 = time_str[2..4].parse().ok()?;
    let seconds: u32 = time_str[4..6].parse().ok()?;

    if hours >= 24 || minutes >= 60 || seconds >= 60 {
        return None;
    }

    Some(hours * 3600 + minutes * 60 + seconds)
}
