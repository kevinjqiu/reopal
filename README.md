# ReoPal: ReoLink Video Manager

ReoPal is a command-line interface (CLI) tool designed to manage and maintain an archive of video files from ReoLink security cameras. It indexes video metadata into a SQLite database, allowing for efficient querying and maintenance tasks, such as enforcing a disk space quota.

## Features

- **Video Indexing**: Scans a directory of ReoLink video files and stores their metadata in a SQLite database.
- **Parallel Processing**: Utilizes multiple CPU cores to scan and process files quickly, making it efficient for large archives.
- **Idempotent Imports**: Prevents duplicate entries in the database, so you can run the import command multiple times without creating redundant data.
- **Disk Quota Management**: A `maintenance` mode helps you keep your video archive from growing too large by deleting the oldest files to stay within a specified quota.
- **Dry Run Mode**: Safely preview which files would be deleted by the maintenance command without actually removing them.
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

ReoPal is operated via subcommands.

### `import`

The `import` subcommand scans a directory and adds the metadata of any new video files to the database.

**Usage:**
```bash
reopal import --directory <path-to-videos> [--db-path <path-to-db>]
```

**Arguments:**
- `-d`, `--directory <path>`: The absolute path to the ReoLink video directory.
- `-b`, `--db-path <path>`: (Optional) The path to the SQLite database file. Defaults to `reopal.db` in the current directory.

**Example:**
```bash
./target/debug/reopal import -d /mnt/reolink/videos
```

### `maintenance`

The `maintenance` subcommand first runs an import to ensure the database is up-to-date, then checks the total size of all non-deleted videos against a specified quota. If the total size exceeds the quota, it will delete the oldest video files until the total size is back within the limit.

**Usage:**
```bash
reopal maintenance --directory <path-to-videos> --quota <size> [--db-path <path>] [--dry-run]
```

**Arguments:**
- `-d`, `--directory <path>`: The path to the ReoLink video directory.
- `-q`, `--quota <size>`: The disk space quota (e.g., "10GB", "500MB", "2.5TB").
- `-b`, `--db-path <path>`: (Optional) The path to the SQLite database file. Defaults to `reopal.db`.
- `--dry-run`: (Flag) If included, the command will only print the files that would be deleted without performing any actual file deletion or database updates.

**Examples:**

**Perform maintenance with a 100GB quota:**
```bash
./target/debug/reopal maintenance -d /mnt/reolink/videos -q 100GB
```

**Perform a dry run to see what would be deleted with a 50GB quota:**
```bash
./target/debug/reopal maintenance -d /mnt/reolink/videos -q 50GB --dry-run
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
