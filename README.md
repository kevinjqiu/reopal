# ReoPal: ReoLink Video Manager

ReoPal is a comprehensive video management system for ReoLink security cameras, featuring both a command-line interface (CLI) and a modern web-based viewer. It indexes video metadata into a SQLite database and provides powerful tools for managing and viewing your camera footage.

## Features

### Core Functionality
- **Video Indexing**: Scans a directory of ReoLink video files and stores their metadata in a SQLite database.
- **Parallel Processing**: Utilizes multiple CPU cores to scan and process files quickly, making it efficient for large archives.
- **Idempotent Imports**: Prevents duplicate entries in the database, so you can run the import command multiple times without creating redundant data.
- **Disk Quota Management**: A `maintenance` mode helps you keep your video archive from growing too large by deleting the oldest files to stay within a specified quota.
- **Dry Run Mode**: Safely preview which files would be deleted by the maintenance command without actually removing them.

### Web Viewer (NEW!)
- **Modern Web Interface**: Browser-based dashboard for viewing and managing camera footage
- **Video Playback**: Stream videos directly in your browser with range request support
- **Advanced Search & Filtering**: Search by camera name, date ranges, and custom criteria
- **Camera Management**: Overview of all cameras with statistics and last recording times
- **Responsive Design**: Works on desktop and mobile devices
- **Real-time Updates**: Dashboard updates automatically when new videos are indexed

### Technical Features
- **Centralized Configuration**: Uses a single YAML file to define all parameters for the `import`, `maintenance`, and `web` commands.
- **Modular Codebase**: Built with a clean, modular structure in Rust for maintainability and future expansion.
- **RESTful API**: Complete API for integration with other tools and services

## Building the Project

To build the project, you will need to have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed.

1.  Clone the repository:
    ```bash
    git clone <repository-url>
    cd reopal
    ```

2.  Build the project:
    ```bash
    cargo build
    ```
    The executable will be available at `target/debug/reopal`. For a release build, use `cargo build --release`.

## Usage

ReoPal is operated via `import`, `maintenance`, and `web` subcommands. All parameters for these commands are sourced from a central YAML configuration file.

**Command:**
```bash
reopal [--config <path-to-config.yml>] <subcommand>
```

**Global Argument:**
- `-c`, `--config <path>`: Path to the YAML configuration file. Defaults to `config.yml`.

### Subcommands

- **`import`**: Scans the video directory and indexes new files.
- **`maintenance`**: First, runs an import to update the database, then enforces the disk quota defined in the configuration.
- **`web`**: Starts the web viewer server for browser-based access to your video archive.

### Configuration File

All settings are managed in a single YAML file (e.g., `config.yml`).

**Example Configuration:**
```yaml
# config.yml

# The root directory where your ReoLink videos are stored.
directory: "/mnt/reolink/videos"

# The path to the SQLite database file.
db_path: "reopal.db"

# Configuration for the 'maintenance' subcommand.
# This section is optional if you only plan to use the 'import' command.
maintenance:
  # The disk space quota (e.g., "100GB", "500MB", "2.5TB").
  quota: "50GB"
  # If true, the command will only print the files that would be deleted.
  dry_run: true

# Configuration for the 'web' subcommand.
web_viewer:
  # Server configuration
  host: "0.0.0.0"
  port: 8080
  
  # Authentication (optional)
  auth:
    enabled: false
    session_timeout: "24h"
    admin_users: ["admin"]
  
  # Video processing (optional)
  thumbnails:
    enabled: true
    cache_dir: "./thumbnails"
    quality: "medium"
    
  # Performance settings (optional)
  max_concurrent_streams: 10
  cache_size: "1GB"
  
  # Feature flags (optional)
  features:
    real_time_updates: true
    bulk_operations: true
    mobile_optimized: true
```

### Running the Commands

**Import videos:**
```bash
./target/debug/reopal import
```

**Run maintenance:**
```bash
./target/debug/reopal maintenance
```

**Start web viewer:**
```bash
./target/debug/reopal web
```

**Use a custom configuration file:**
```bash
./target/debug/reopal --config /path/to/my/config.yml web
```

## Web Viewer Usage

Once you start the web viewer with `reopal web`, you can access it at `http://localhost:8080` (or the configured host/port).

### Features:
- **Dashboard**: Overview of your video archive with statistics and recent recordings
- **Videos**: Browse all videos with pagination, filtering, and search capabilities
- **Cameras**: View camera-specific statistics and recordings
- **Search**: Find specific videos using text search and date/time filters
- **Video Player**: Stream videos directly in your browser with seek controls
- **Manual Refresh**: Update video metadata on-demand with the refresh button or Ctrl+R/F5

### API Endpoints:
- `GET /api/videos` - List videos with pagination and filtering
- `GET /api/videos/:id` - Get specific video metadata
- `GET /api/videos/:id/stream` - Stream video file
- `POST /api/videos/search` - Search videos
- `GET /api/cameras` - List cameras with statistics
- `POST /api/import` - Manually refresh video metadata
- `GET /api/health` - Health check endpoint

## Database Schema

The tool creates a SQLite database with a single table named `videos`.

| Column        | Type    | Description                               |
|---------------|---------|-------------------------------------------|
| `file_path`   | TEXT    | **Primary Key.** The full path to the video file. |
| `camera_name` | TEXT    | The name of the camera.                   |
| `date`        | TEXT    | The date of the recording (MMDDYYYY).     |
| `start_time`  | TEXT    | The start time of the recording (HHMMSS). |
| `end_time`    | TEXT    | The end time of the recording (HHMMSS).   |
| `file_size`   | INTEGER | The size of the file in bytes.            |
| `deleted`     | BOOLEAN | `true` if the file has been deleted.      |

## Project Structure

```
reopal/
├── src/
│   ├── main.rs          # Application entry point
│   ├── cli.rs           # Command-line interface and configuration
│   ├── db.rs            # Database operations
│   ├── models.rs        # Data structures
│   ├── scanner.rs       # Video file scanning
│   ├── maintenance.rs   # Maintenance operations
│   └── web/             # Web viewer components
│       ├── mod.rs       # Web module exports
│       ├── server.rs    # Web server implementation
│       ├── routes.rs    # HTTP routes
│       ├── handlers.rs  # Request handlers
│       ├── middleware.rs # Middleware components
│       └── state.rs     # Application state
├── web/
│   └── static/          # Frontend assets
│       ├── index.html   # Main web interface
│       ├── styles.css   # Styling
│       └── app.js       # Frontend JavaScript
├── docs/
│   └── WEB_VIEWER_PRD.md # Product requirements document
├── config.yml           # Configuration file
└── README.md           # This file
```

## Development

The project is built with Rust and uses modern web technologies:

- **Backend**: Rust with Axum web framework
- **Database**: SQLite with rusqlite
- **Frontend**: Vanilla HTML/CSS/JavaScript
- **Async Runtime**: Tokio
- **HTTP Client**: Tower and Hyper

### Contributing

1. Follow Rust best practices and run `cargo fmt` before committing
2. Ensure all tests pass with `cargo test`
3. Update documentation when adding new features
4. Consider security implications, especially for web-facing features

## License

[Add your license information here]
