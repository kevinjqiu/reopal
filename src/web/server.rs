use axum::{serve, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

use crate::web::{routes, AppState};

pub struct WebServer {
    state: AppState,
}

impl WebServer {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn start(self, host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let app = self.create_app();

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let listener = TcpListener::bind(addr).await?;

        println!("🚀 ReoPal Web Viewer starting on http://{}:{}", host, port);
        println!("📹 Serving videos from: {}", self.state.config.directory);

        serve(listener, app).await?;

        Ok(())
    }

    fn create_app(&self) -> Router {
        Router::new()
            .nest("/api", routes::create_api_routes())
            .nest_service("/", ServeDir::new("web/static"))
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http())
            .with_state(self.state.clone())
    }
}
