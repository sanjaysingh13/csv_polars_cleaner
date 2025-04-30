# csv_polars_cleaner

A robust Rust library for extracting and cleaning tabular data from messy CSV files using the Polars DataFrame engine.

## Objective

- To reliably parse CSV files that may contain metadata, comments, empty lines, or other non-tabular content before or after the actual data table.
- To automatically detect the start and end of the true data region using statistical heuristics (mode of column counts).

## Functionality

- Skips metadata, comments, and blank lines to find the real table header and data.
- Uses the most frequent column count to infer the bounds of the data block.
- Returns a Polars DataFrame for further analysis or processing.
- Provides clear error messages for malformed or unsupported files.

## Limitations

- Only supports single-table CSVs (not multi-table or hierarchical data).
- Assumes the delimiter is consistent within the data region (default: `,`).
- Does not attempt to infer or repair rows with inconsistent column counts within the main data region.
- Metadata and comments must not contain the delimiter in a way that mimics a table row.

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
csv_polars_cleaner = "<version>"
```

Example usage:
```rust
use csv_polars_cleaner::parse_file;

fn main() {
    let path = "path/to/your.csv";
    match parse_file(path, b',') {
        Ok(df) => {
            println!("Headers: {:?}", df.get_column_names());
            println!("Number of rows: {}", df.height());
        }
        Err(e) => {
            eprintln!("Failed to parse file: {:?}", e);
        }
    }
}
```

## Command-line Usage

This crate includes a simple CLI for quickly checking CSV parsing on your system:

```sh
cargo run -- path/to/your.csv
```

---

For more details, see the source code.

[**View API Documentation (GitHub Pages)**](https://sanjaysingh13.github.io/csv_polars_cleaner/csv_polars_cleaner/)
