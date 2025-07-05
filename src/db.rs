use crate::models::VideoRecording;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};

/// Initializes the database and creates the 'videos' table if it doesn't exist.
pub fn init_db(conn: &Connection) -> Result<()> {
    // First, check if we need to migrate the table
    let mut has_old_schema = false;
    let stmt = conn.prepare("PRAGMA table_info(videos)");
    if let Ok(mut stmt) = stmt {
        let rows = stmt.query_map([], |row| {
            let column_name: String = row.get(1)?;
            let column_type: String = row.get(2)?;
            Ok((column_name, column_type))
        });

        if let Ok(rows) = rows {
            for (name, type_name) in rows.flatten() {
                if (name == "start_time" || name == "end_time") && type_name == "TEXT" {
                    has_old_schema = true;
                    break;
                }
            }
        }
    }

    if has_old_schema {
        // Migrate existing table
        conn.execute("ALTER TABLE videos RENAME TO videos_old", [])?;
        conn.execute(
            "CREATE TABLE videos (
                file_path TEXT PRIMARY KEY,
                camera_name TEXT NOT NULL,
                date TEXT NOT NULL,
                start_time DATETIME NOT NULL,
                end_time DATETIME NOT NULL,
                file_size INTEGER NOT NULL,
                deleted BOOLEAN NOT NULL DEFAULT 0
            )",
            [],
        )?;

        // Migrate data from old table to new table
        conn.execute(
            "INSERT INTO videos (file_path, camera_name, date, start_time, end_time, file_size, deleted)
             SELECT file_path, camera_name, date, datetime(start_time), datetime(end_time), file_size, deleted
             FROM videos_old",
            [],
        )?;

        // Drop the old table
        conn.execute("DROP TABLE videos_old", [])?;
    } else {
        // Create new table with DATETIME columns
        conn.execute(
            "CREATE TABLE IF NOT EXISTS videos (
                file_path TEXT PRIMARY KEY,
                camera_name TEXT NOT NULL,
                date TEXT NOT NULL,
                start_time DATETIME NOT NULL,
                end_time DATETIME NOT NULL,
                file_size INTEGER NOT NULL,
                deleted BOOLEAN NOT NULL DEFAULT 0
            )",
            [],
        )?;
    }
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

/// Retrieves all non-deleted video recordings, sorted by date and start time.
pub fn get_all_non_deleted_recordings(conn: &Connection) -> Result<Vec<VideoRecording>> {
    let mut stmt = conn.prepare(
        "SELECT file_path, camera_name, date, start_time, end_time, file_size, deleted
         FROM videos
         WHERE deleted = 0
         ORDER BY date, start_time",
    )?;
    let record_iter = stmt.query_map([], |row| {
        let start_time: DateTime<Utc> = row.get(3)?;
        let end_time: DateTime<Utc> = row.get(4)?;

        Ok(VideoRecording {
            file_path: row.get(0)?,
            camera_name: row.get(1)?,
            date: row.get(2)?,
            start_time,
            end_time,
            file_size: row.get(5)?,
            deleted: row.get(6)?,
        })
    })?;

    let mut records = Vec::new();
    for record in record_iter {
        records.push(record?);
    }
    Ok(records)
}

/// Marks a video recording as deleted in the database.
pub fn mark_as_deleted(conn: &Connection, file_path: &str) -> Result<usize> {
    conn.execute(
        "UPDATE videos SET deleted = 1 WHERE file_path = ?1",
        [file_path],
    )
}
