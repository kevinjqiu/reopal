#[derive(Debug)]
pub struct VideoRecording {
    pub camera_name: String,
    pub date: String,
    pub start_time: String,
    pub end_time: String,
    pub file_path: String,
    pub file_size: u64,
    pub deleted: bool,
}
