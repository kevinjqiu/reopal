use axum::{
    routing::{get, post},
    Router,
};

use crate::web::{handlers, AppState};

pub fn create_api_routes() -> Router<AppState> {
    Router::new()
        .route("/videos", get(handlers::list_videos))
        .route("/videos/:id", get(handlers::get_video))
        .route("/videos/:id/stream", get(handlers::stream_video))
        .route("/videos/search", post(handlers::search_videos))
        .route("/cameras", get(handlers::list_cameras))
        .route("/import", post(handlers::manual_import))
        .route("/health", get(handlers::health_check))
}
