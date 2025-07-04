use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use crate::web::AppState;

#[derive(Deserialize)]
pub struct VideoQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub camera: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

#[derive(Serialize)]
pub struct VideoResponse {
    pub id: String,
    pub camera_name: String,
    pub date: String,
    pub start_time: String,
    pub end_time: String,
    pub file_size: u64,
    pub file_path: String,
    pub deleted: bool,
}

#[derive(Serialize)]
pub struct VideosListResponse {
    pub videos: Vec<VideoResponse>,
    pub total: u32,
    pub page: u32,
    pub limit: u32,
}

#[derive(Serialize)]
pub struct CameraResponse {
    pub name: String,
    pub video_count: u32,
    pub last_recording: Option<String>,
}

#[derive(Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub camera: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

/// Helper function to format DateTime to HHMMSS string
fn format_time_to_string(datetime: &DateTime<Utc>) -> String {
    datetime.format("%H%M%S").to_string()
}

/// Helper function to convert ISO date format (YYYY-MM-DD) to database format (MMDDYYYY)
fn convert_iso_date_to_db_format(iso_date: &str) -> Option<String> {
    let date = NaiveDate::parse_from_str(iso_date, "%Y-%m-%d").ok()?;
    Some(date.format("%m%d%Y").to_string())
}

/// List all videos with pagination and filtering
pub async fn list_videos(
    State(state): State<AppState>,
    Query(params): Query<VideoQuery>,
) -> Result<Json<VideosListResponse>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let offset = (page - 1) * limit;

    let db = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Build the WHERE clause for both count and data queries
    let mut where_clause = "WHERE 1=1".to_string();
    let mut conditions = Vec::new();

    if let Some(camera) = &params.camera {
        where_clause.push_str(" AND camera_name = ?");
        conditions.push(camera.clone());
    }

    if let Some(date_from) = &params.date_from {
        if let Some(converted_date) = convert_iso_date_to_db_format(date_from) {
            where_clause.push_str(" AND date >= ?");
            conditions.push(converted_date);
        }
    }

    if let Some(date_to) = &params.date_to {
        if let Some(converted_date) = convert_iso_date_to_db_format(date_to) {
            where_clause.push_str(" AND date < ?");
            conditions.push(converted_date);
        }
    }

    // First, get the total count
    let count_query = format!("SELECT COUNT(*) FROM videos {}", where_clause);
    let mut count_stmt = db
        .prepare(&count_query)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let total: u32 = count_stmt
        .query_row(rusqlite::params_from_iter(conditions.iter()), |row| {
            row.get::<_, u32>(0)
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Build the main query
    let mut query = format!("SELECT file_path, camera_name, date, start_time, end_time, file_size, deleted FROM videos {}", where_clause);
    query.push_str(" ORDER BY date DESC, start_time DESC");

    // Add pagination if limit is reasonable (not trying to get all records)
    if limit > 0 && limit <= 10000 {
        query.push_str(" LIMIT ? OFFSET ?");
        conditions.push(limit.to_string());
        conditions.push(offset.to_string());
    }

    let mut stmt = db
        .prepare(&query)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let video_iter = stmt
        .query_map(rusqlite::params_from_iter(conditions.iter()), |row| {
            let start_time: DateTime<Utc> = row.get(3)?;
            let end_time: DateTime<Utc> = row.get(4)?;

            Ok(VideoResponse {
                id: generate_video_id(row.get::<_, String>(0)?),
                camera_name: row.get(1)?,
                date: row.get(2)?,
                start_time: format_time_to_string(&start_time),
                end_time: format_time_to_string(&end_time),
                file_size: row.get(5)?,
                file_path: row.get(0)?,
                deleted: row.get(6)?,
            })
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let videos: Result<Vec<VideoResponse>, _> = video_iter.collect();
    let videos = videos.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(VideosListResponse {
        videos,
        total,
        page,
        limit,
    }))
}

/// Get specific video metadata
pub async fn get_video(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<VideoResponse>, StatusCode> {
    let db = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut stmt = db
        .prepare("SELECT file_path, camera_name, date, start_time, end_time, file_size, deleted FROM videos WHERE file_path = ?")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let file_path = decode_video_id(id);
    let video = stmt
        .query_row([&file_path], |row| {
            let start_time: DateTime<Utc> = row.get(3)?;
            let end_time: DateTime<Utc> = row.get(4)?;

            Ok(VideoResponse {
                id: generate_video_id(row.get::<_, String>(0)?),
                camera_name: row.get(1)?,
                date: row.get(2)?,
                start_time: format_time_to_string(&start_time),
                end_time: format_time_to_string(&end_time),
                file_size: row.get(5)?,
                file_path: row.get(0)?,
                deleted: row.get(6)?,
            })
        })
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(video))
}

/// Stream video file with range support
pub async fn stream_video(
    State(state): State<AppState>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> Result<Response, StatusCode> {
    let file_path = decode_video_id(id);
    let full_path = PathBuf::from(&state.config.directory).join(&file_path);

    // Check if file exists
    if !full_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let mut file = File::open(&full_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let file_size = file
        .metadata()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .len();

    // Handle range requests
    if let Some(range_header) = headers.get("range") {
        let range_str = range_header.to_str().map_err(|_| StatusCode::BAD_REQUEST)?;

        if let Some(range) = parse_range_header(range_str, file_size) {
            let content_length = range.1 - range.0 + 1;
            let mut buffer = vec![0u8; content_length as usize];

            file.seek(SeekFrom::Start(range.0))
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            file.read_exact(&mut buffer)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            return Ok((
                StatusCode::PARTIAL_CONTENT,
                [
                    (header::CONTENT_TYPE, "video/mp4"),
                    (header::CONTENT_LENGTH, &content_length.to_string()),
                    (
                        header::CONTENT_RANGE,
                        &format!("bytes {}-{}/{}", range.0, range.1, file_size),
                    ),
                    (header::ACCEPT_RANGES, "bytes"),
                ],
                buffer,
            )
                .into_response());
        }
    }

    // Full file response
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "video/mp4"),
            (header::CONTENT_LENGTH, &file_size.to_string()),
            (header::ACCEPT_RANGES, "bytes"),
        ],
        buffer,
    )
        .into_response())
}

/// Search videos by query
pub async fn search_videos(
    State(state): State<AppState>,
    Json(search_req): Json<SearchRequest>,
) -> Result<Json<Vec<VideoResponse>>, StatusCode> {
    let db = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut query = "SELECT file_path, camera_name, date, start_time, end_time, file_size, deleted FROM videos WHERE (camera_name LIKE ? OR file_path LIKE ?)".to_string();
    let mut conditions = vec![
        format!("%{}%", search_req.query),
        format!("%{}%", search_req.query),
    ];

    if let Some(camera) = &search_req.camera {
        query.push_str(" AND camera_name = ?");
        conditions.push(camera.clone());
    }

    if let Some(date_from) = &search_req.date_from {
        if let Some(converted_date) = convert_iso_date_to_db_format(date_from) {
            query.push_str(" AND date >= ?");
            conditions.push(converted_date);
        }
    }

    if let Some(date_to) = &search_req.date_to {
        if let Some(converted_date) = convert_iso_date_to_db_format(date_to) {
            query.push_str(" AND date < ?");
            conditions.push(converted_date);
        }
    }

    query.push_str(" ORDER BY date DESC, start_time DESC");

    let mut stmt = db
        .prepare(&query)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let video_iter = stmt
        .query_map(rusqlite::params_from_iter(conditions.iter()), |row| {
            let start_time: DateTime<Utc> = row.get(3)?;
            let end_time: DateTime<Utc> = row.get(4)?;

            Ok(VideoResponse {
                id: generate_video_id(row.get::<_, String>(0)?),
                camera_name: row.get(1)?,
                date: row.get(2)?,
                start_time: format_time_to_string(&start_time),
                end_time: format_time_to_string(&end_time),
                file_size: row.get(5)?,
                file_path: row.get(0)?,
                deleted: row.get(6)?,
            })
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let videos: Result<Vec<VideoResponse>, _> = video_iter.collect();
    let videos = videos.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(videos))
}

/// List all cameras
pub async fn list_cameras(
    State(state): State<AppState>,
) -> Result<Json<Vec<CameraResponse>>, StatusCode> {
    let db = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut stmt = db
        .prepare("SELECT camera_name, COUNT(*) as video_count, MAX(date || start_time) as last_recording FROM videos GROUP BY camera_name ORDER BY camera_name")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let camera_iter = stmt
        .query_map([], |row| {
            Ok(CameraResponse {
                name: row.get(0)?,
                video_count: row.get(1)?,
                last_recording: row.get(2)?,
            })
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cameras: Result<Vec<CameraResponse>, _> = camera_iter.collect();
    let cameras = cameras.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(cameras))
}

/// Health check endpoint
pub async fn health_check() -> Json<HashMap<String, String>> {
    let mut response = HashMap::new();
    response.insert("status".to_string(), "healthy".to_string());
    response.insert("service".to_string(), "reopal-web-viewer".to_string());
    Json(response)
}

/// Manual import/refresh endpoint
pub async fn manual_import(
    State(state): State<AppState>,
) -> Result<Json<HashMap<String, String>>, StatusCode> {
    let db = state
        .db
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Release the database lock before calling scanner
    drop(db);

    // Call the scanner function to import new videos
    match crate::scanner::scan_directory(&state.config.directory, &state.db.lock().unwrap()) {
        Ok(()) => {
            let mut response = HashMap::new();
            response.insert("status".to_string(), "success".to_string());
            response.insert(
                "message".to_string(),
                "Video metadata refreshed successfully".to_string(),
            );
            Ok(Json(response))
        }
        Err(e) => {
            eprintln!("Import error: {}", e);
            let mut response = HashMap::new();
            response.insert("status".to_string(), "error".to_string());
            response.insert("message".to_string(), format!("Import failed: {}", e));
            Ok(Json(response))
        }
    }
}

// Helper functions
fn generate_video_id(file_path: String) -> String {
    use base64::prelude::*;
    BASE64_STANDARD.encode(file_path.as_bytes())
}

fn decode_video_id(id: String) -> String {
    use base64::prelude::*;
    String::from_utf8(BASE64_STANDARD.decode(id).unwrap_or_default()).unwrap_or_default()
}

fn parse_range_header(range_header: &str, file_size: u64) -> Option<(u64, u64)> {
    if !range_header.starts_with("bytes=") {
        return None;
    }

    let range_part = &range_header[6..];
    let parts: Vec<&str> = range_part.split('-').collect();

    if parts.len() != 2 {
        return None;
    }

    let start = if parts[0].is_empty() {
        0
    } else {
        parts[0].parse::<u64>().ok()?
    };

    let end = if parts[1].is_empty() {
        file_size - 1
    } else {
        parts[1].parse::<u64>().ok()?
    };

    Some((start, end))
}
