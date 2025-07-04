# ReoPal CLI

## Core Functionality: Video Indexing

Create a CLI command that can read a ReoLink video directory and index the video files into a SQLite database.

### Directory and File Structure

The ReoLink video directory has the following format:
- The date-stamped folders are in the format of `MMDDYYYY`.
- Inside each folder, there are multiple video files.
- The video files are in the format of `<camera name>-00-<start time>-<end time>.mp4`.
- `<start time>` and `<end time>` are in the format of `HHMMSS`.
- Both times are in the local timezone.

### Database Schema

Store each file's metadata in a SQLite database with the following fields extracted:
- `file_path` (TEXT, PRIMARY KEY)
- `camera_name` (TEXT)
- `date` (TEXT)
- `start_time` (TEXT)
- `end_time` (TEXT)
- `file_size` (INTEGER)
- `deleted` (BOOLEAN, default `false`)

## Implemented Enhancements

- **Subcommand Interface:** The core indexing functionality is exposed through an `import` subcommand.
- **Parallel Processing:** The file scanning and parsing are executed in parallel to improve performance, especially for large directories.
- **Idempotent Imports:** The `file_path` is used as the primary key in the database. This ensures that running the import command multiple times on the same directory will not create duplicate records.
- **Modular Codebase:** The project is structured with a library crate that contains the core logic, separated into modules for the CLI, database, data models, and file scanning. This promotes code reuse and maintainability.