use crate::db;
use byte_unit::Byte;
use rusqlite::Connection;
use std::fs;
use std::str::FromStr;

pub fn run_maintenance(
    conn: &Connection,
    quota_str: &str,
    dry_run: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let quota = Byte::from_str(quota_str)?.as_u64();
    let recordings = db::get_all_non_deleted_recordings(conn)?;

    let total_size: u64 = recordings.iter().map(|r| r.file_size).sum();

    if total_size <= quota {
        println!("Total size is within the quota. No action needed.");
        return Ok(());
    }

    let mut size_to_delete = total_size - quota;
    let mut recordings_to_delete = Vec::new();

    for recording in recordings {
        if size_to_delete == 0 {
            break;
        }
        if recording.file_size >= size_to_delete {
            size_to_delete = 0;
        } else {
            size_to_delete -= recording.file_size;
        }
        recordings_to_delete.push(recording);
    }

    if dry_run {
        println!("-- Dry Run --");
        println!("The following files would be deleted:");
        for recording in &recordings_to_delete {
            println!("- {} ({} bytes)", recording.file_path, recording.file_size);
        }
    } else {
        let tx = conn.unchecked_transaction()?;
        for recording in &recordings_to_delete {
            println!("Deleting: {}", recording.file_path);
            fs::remove_file(&recording.file_path)?;
            db::mark_as_deleted(&tx, &recording.file_path)?;
        }
        tx.commit()?;
        println!("Maintenance complete.");
    }

    Ok(())
}
