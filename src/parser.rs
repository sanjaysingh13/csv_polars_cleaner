use anyhow::{Context, Result};
use polars::datatypes::DataType;
use polars::prelude::*;
use std::borrow::Cow;
use std::io::Cursor;

/// Return Polars DataFrame using box logic, preserving internal newlines
pub fn parse_boxed_data(raw: &str, delimiter: u8) -> Result<DataFrame> {
    // Normalize line endings and split into lines
    let normalized = crate::preprocessor::normalize_line_endings(raw);
    let mut lines: Vec<&str> = normalized.lines().collect();

    // Remove empty lines
    lines.retain(|l| !l.trim().is_empty());

    // Detect the data bounds, skipping comments and empty lines
    let (start, end) = crate::detector::detect_data_bounds(&lines, delimiter as char)
        .context("Could not detect valid data box")?;

    // Preprocess the boxed data: trim whitespace from each field and repair malformed rows
    let header = lines[start]
        .split(delimiter as char)
        .map(|s| s.trim())
        .collect::<Vec<_>>();
    let num_cols = header.len();

    let mut processed_lines = vec![header.join(&String::from(delimiter as char))];
    for line in &lines[start + 1..=end] {
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }
        let mut fields: Vec<String> = line
            .split(delimiter as char)
            .map(|s| s.trim().to_string())
            .collect();
        // If not enough fields, try splitting by whitespace
        if fields.len() < num_cols {
            let ws_fields: Vec<String> = line
                .split_whitespace()
                .map(|s| s.trim().to_string())
                .collect();
            if ws_fields.len() == num_cols {
                fields = ws_fields;
            } else if ws_fields.len() > num_cols && num_cols > 1 {
                // Merge trailing fields into the last column
                let mut merged: Vec<String> = ws_fields[..num_cols - 1].to_vec();
                merged.push(ws_fields[num_cols - 1..].join(" "));
                fields = merged;
            }
        }
        // Pad with empty if still not enough fields
        while fields.len() < num_cols {
            fields.push(String::new());
        }
        processed_lines.push(fields.join(&String::from(delimiter as char)));
    }

    // Join the processed lines
    let boxed = processed_lines.join("\n");

    // Create a Cursor from the string bytes
    let cursor = Cursor::new(boxed.as_bytes());

    // Configure CSV parsing options
    let parse_options = CsvParseOptions::default()
        .with_separator(delimiter)
        .with_try_parse_dates(false)
        .with_missing_is_null(true)
        .with_comment_prefix(Some("#"));

    let read_options = CsvReadOptions::default()
        .with_parse_options(parse_options)
        .with_has_header(true)
        .with_skip_rows(0);

    let df = read_options.into_reader_with_file_handle(cursor).finish()?;

    // Convert all columns to String (string)
    let df = df
        .get_columns()
        .iter()
        .map(|s| s.cast(&DataType::String))
        .collect::<polars::prelude::PolarsResult<Vec<_>>>()?;
    let df = DataFrame::new(df)?;

    // Clean: Remove matching leading/trailing quotes from all string columns
    let mut cleaned_cols = Vec::with_capacity(df.width());
    for s in df.get_columns() {
        if s.dtype() == &DataType::String {
            let cleaned = s.str()
                .unwrap()
                .apply(|val: Option<&str>| {
                    val.map(|v| {
                        let bytes = v.as_bytes();
                        // Strip single quotes if present at both ends
                        if bytes.len() >= 2 && bytes[0] == b'\'' && bytes[bytes.len()-1] == b'\'' {
                            let stripped = &v[1..v.len()-1];
                            if stripped.eq_ignore_ascii_case("null") {
                                return Cow::Borrowed("");
                            }
                            return Cow::Owned(stripped.to_string());
                        }
                        if v.eq_ignore_ascii_case("null") {
                            return Cow::Borrowed("");
                        }
                        Cow::Borrowed(v)
                    })
                })
                .into_series()
                .into();
            cleaned_cols.push(cleaned);
        } else {
            cleaned_cols.push(s.clone().into());
        }
    }
    let df = DataFrame::new(cleaned_cols)?;

    Ok(df)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a test DataFrame
    fn create_expected_df() -> DataFrame {
        df!(
            "name" => &["Alice", "Bob",  "Charlie",],
            "age" => &["30", "25", "35"],
            "city" => &["New York", "London", "Paris"]
        )
        .unwrap()
    }

    #[test]
    fn test_mixed_line_endings() {
        // Create a malformed CSV with mixed line endings and metadata
        let input = r#"# This is a comment
# Another comment
# Metadata: version=1.0

name,age,city
Alice,30,New York
Bob,25,London
Charlie,35,Paris

# End of file"#;

        let result = parse_boxed_data(input, b',');
        assert!(
            result.is_ok(),
            "Failed to parse CSV with mixed line endings"
        );

        let df = result.unwrap();
        let expected = create_expected_df();
        assert!(df.equals(&expected), "DataFrame content mismatch");
    }

    #[test]
    fn test_extra_whitespace() {
        // Create a malformed CSV with extra whitespace
        let input = r#"  name  ,  age  ,  city  
  Alice  ,  30  ,  New York  
  Bob  ,  25  ,  London  
  Charlie  ,  35  ,  Paris  "#;

        let result = parse_boxed_data(input, b',');
        assert!(result.is_ok(), "Failed to parse CSV with extra whitespace");

        let df = result.unwrap();
        let expected = create_expected_df();
        assert!(df.equals(&expected), "DataFrame content mismatch");
    }

    #[test]
    fn test_empty_lines() {
        // Create a malformed CSV with empty lines
        let input = r#"

name,age,city

Alice,30,New York

Bob,25,London

Charlie,35,Paris

"#;

        let result = parse_boxed_data(input, b',');
        assert!(result.is_ok(), "Failed to parse CSV with empty lines");

        let df = result.unwrap();
        let expected = create_expected_df();
        if !df.equals(&expected) {
            println!("EXPECTED:\n{:?}", expected);
            println!("ACTUAL:\n{:?}", df);
        }
        assert!(df.equals(&expected), "DataFrame content mismatch");
    }

    #[test]
    fn test_tab_delimiter() {
        // Create a malformed CSV with tab delimiter
        let input = r#"name	age	city
Alice	30	New York
Bob	25	London
Charlie	35	Paris"#;

        let result = parse_boxed_data(input, b'\t');
        assert!(result.is_ok(), "Failed to parse CSV with tab delimiter");

        let df = result.unwrap();
        let expected = create_expected_df();
        assert!(df.equals(&expected), "DataFrame content mismatch");
    }
}
