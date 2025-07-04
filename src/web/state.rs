use crate::cli::Config;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

/// Shared application state for the web server
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new(db: Connection, config: Config) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            config: Arc::new(config),
        }
    }
}
