# ReoPal: ReoLink Video Manager

ReoPal is a command-line interface (CLI) tool designed to manage and maintain an archive of video files from ReoLink security cameras. It indexes video metadata into a SQLite database, allowing for efficient querying and maintenance tasks, such as enforcing a disk space quota.

## Features

- **Video Indexing**: Scans a directory of ReoLink video files and stores their metadata in a SQLite database.
- **Parallel Processing**: Utilizes multiple CPU cores to scan and process files quickly, making it efficient for large archives.
- **Idempotent Imports**: Prevents duplicate entries in the database, so you can run the import command multiple times without creating redundant data.
- **Disk Quota Management**: A `maintenance` mode helps you keep your video archive from growing too large by deleting the oldest files to stay within a specified quota.
- **Dry Run Mode**: Safely preview which files would be deleted by the maintenance command without actually removing them.
- **Centralized Configuration**: Uses a single YAML file to define all parameters for the `import` and `maintenance` commands.
- **Modular Codebase**: Built with a clean, modular structure in Rust for maintainability and future expansion.

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

ReoPal is operated via `import` and `maintenance` subcommands. All parameters for these commands are sourced from a central YAML configuration file.

**Command:**
```bash
reopal [--config <path-to-config.yml>] <subcommand>
```

**Global Argument:**
- `-c`, `--config <path>`: Path to the YAML configuration file. Defaults to `config.yml`.

### Subcommands

- **`import`**: Scans the video directory and indexes new files.
- **`maintenance`**: First, runs an import to update the database, then enforces the disk quota defined in the configuration.

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

**Use a custom configuration file:**
```bash
./target/debug/reopal --config /path/to/my/config.yml maintenance
```

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
