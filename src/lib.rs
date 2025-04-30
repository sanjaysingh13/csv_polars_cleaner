//! # csv_polars_cleaner
//!
//! A robust Rust library for extracting and cleaning tabular data from messy CSV files using the Polars DataFrame engine.
//!
//! ## Features
//! - Automatically skips metadata, comments, and blank lines.
//! - Detects the real data region using the mode of column counts.
//! - Returns a Polars DataFrame for further analysis.
//!
//! ## Example
//!
//! ```no_run
//! use csv_polars_cleaner::parse_file;
//! // This example will fail unless "data.csv" exists.
//! let df = parse_file("data.csv", b',');
//! match df {
//!     Ok(df) => println!("Headers: {:?}", df.get_column_names()),
//!     Err(e) => eprintln!("Failed to parse: {e}"),
//! }
//! ```

pub mod preprocessor;
pub mod detector;
pub mod parser;

use polars::prelude::*;
use anyhow::Result;
use std::fs;

/// Parse CSV data from a string slice and return a Polars DataFrame.
///
/// This function is re-exported from the `parser` module for convenience.
pub use parser::parse_boxed_data;

/// Load and parse a single malformed CSV file.
///
/// This function reads a CSV file from disk, skips metadata and comments,
/// detects the data region, and parses it into a Polars DataFrame.
///
/// # Arguments
/// * `path` - Path to the CSV file.
/// * `delimiter` - The delimiter as a byte (e.g., `b','`).
///
/// # Errors
/// Returns an error if the file cannot be read or parsed.
///
/// # Example
/// ```no_run
/// use csv_polars_cleaner::parse_file;
/// let df = parse_file("data.csv", b',');
/// match df {
///     Ok(df) => println!("Headers: {:?}", df.get_column_names()),
///     Err(e) => eprintln!("Failed to parse: {e}"),
/// }
/// ```
pub fn parse_file(path: &str, delimiter: u8) -> Result<DataFrame> {
    let content = fs::read_to_string(path)?;
    parser::parse_boxed_data(&content, delimiter)
}

/// Parse all CSV files in a folder into a vector of Polars DataFrames.
///
/// This function uses a glob pattern to find files, parses each file,
/// and returns a vector of DataFrames. Errors in individual files are reported to stderr.
///
/// # Arguments
/// * `glob_path` - Glob pattern for file selection (e.g., "data/*.csv").
/// * `delimiter` - The delimiter as a byte (e.g., `b','`).
///
/// # Returns
/// A vector of successfully parsed DataFrames.
///
/// # Example
/// ```no_run
/// use csv_polars_cleaner::parse_folder;
/// let dfs = parse_folder("data/*.csv", b',');
/// match dfs {
///     Ok(dfs) => println!("Parsed {} files", dfs.len()),
///     Err(e) => eprintln!("Failed to parse folder: {e}"),
/// }
/// ```
pub fn parse_folder(glob_path: &str, delimiter: u8) -> Result<Vec<DataFrame>> {
    let mut dfs = vec![];
    for entry in glob::glob(glob_path)? {
        let path = entry?;
        match parse_file(path.to_str().unwrap(), delimiter) {
            Ok(df) => dfs.push(df),
            Err(e) => eprintln!("Failed to parse {}: {:?}", path.display(), e),
        }
    }
    Ok(dfs)
}
