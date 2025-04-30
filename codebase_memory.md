# Codebase Memory: csv_polars_cleaner

## Project Structure
- **src/**
  - `lib.rs`: Main library file, re-exports modules and provides folder/file parsing functions.
  - `parser.rs`: Contains `parse_boxed_data` for robust CSV parsing (handles malformed/dirty data). Includes extensive tests.
  - `detector.rs`: Provides `detect_data_bounds`, used to find structured data bounds in CSV-like files. Includes tests.
  - `preprocessor.rs`: Contains `normalize_line_endings` utility.
  - `utils.rs`: (File not found; possibly missing or empty.)
- **tests/**
  - `test_parser.rs`: Integration-style tests for `parse_boxed_data` covering many malformed CSV scenarios.
- **Cargo.toml/Cargo.lock**: Rust project configuration and dependency lock file.

## Key Functionality
- **parse_file / parse_folder** (in `lib.rs`):
  - Parse single/multiple CSV files using robust logic, returning `DataFrame`(s).
- **parse_boxed_data** (in `parser.rs`):
  - Cleans, normalizes, and parses CSV content, handling comments, whitespace, missing delimiters, and more. Returns all columns as Utf8 strings.
- **detect_data_bounds** (in `detector.rs`):
  - Finds the start/end of actual data in a file, skipping comments/empty lines.
- **normalize_line_endings** (in `preprocessor.rs`):
  - Converts all line endings to `\n` for consistency.

## Testing
- Unit and integration tests are provided for all core parsing logic, focusing on edge cases and malformed data.
- `tests/test_parser.rs` extends coverage with additional real-world scenarios.

## Dependencies
- `polars`: DataFrame library for Rust (main data structure).
- `anyhow`: Error handling.
- `glob`: File path globbing.

## Notes
- The codebase is designed for robust, fault-tolerant CSV parsing, especially for dirty or inconsistent files.
- All DataFrame columns are cast to Utf8 for uniformity.
- Comments and metadata in CSVs are gracefully ignored.
