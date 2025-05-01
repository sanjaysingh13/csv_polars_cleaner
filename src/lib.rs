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
use walkdir::WalkDir;

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

/// Parse all CSV files in a folder and its subfolders into a vector of Polars DataFrames.
///
/// This function recursively searches for .csv files in the root folder,
/// parses each file, and returns a vector of DataFrames. Errors in individual files are reported to stderr.
///
/// # Arguments
/// * `root_folder` - Path to the root folder to search for .csv files (e.g., "data/").
/// * `delimiter` - The delimiter as a byte (e.g., `b','`).
///
/// # Returns
/// A vector of successfully parsed DataFrames.
///
/// # Example
/// ```no_run
/// use csv_polars_cleaner::parse_folder;
/// let dfs = parse_folder("data/", b',');
/// match dfs {
///     Ok(dfs) => println!("Parsed {} files", dfs.len()),
///     Err(e) => eprintln!("Failed to parse folder: {e}"),
/// }
/// ```
pub fn parse_folder(root_folder: &str, delimiter: u8) -> Result<Vec<DataFrame>> {
    
    let mut dfs = vec![];
    for entry in WalkDir::new(root_folder).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "csv") {
            match parse_file(path.to_str().unwrap(), delimiter) {
                Ok(df) => dfs.push(df),
                Err(e) => eprintln!("Failed to parse {}: {:?}", path.display(), e),
            }
        }
    }
    Ok(dfs)
}
