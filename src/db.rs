use crate::models::VideoRecording;
use rusqlite::{Connection, Result};

/// Initializes the database and creates the 'videos' table if it doesn't exist.
pub fn init_db(conn: &Connection) -> Result<()> {
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

/// Inserts a single VideoRecording into the database, ignoring duplicates.
pub fn insert_record(conn: &Connection, record: &VideoRecording) -> Result<usize> {
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
