use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct VideoRecording {
    pub camera_name: String,
    pub date: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub file_path: String,
    pub file_size: u64,
    pub deleted: bool,
}
