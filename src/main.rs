use csv_polars_cleaner::parse_folder;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <csv_folder>", args[0]);
        std::process::exit(1);
    }
    let folder = &args[1];
    match parse_folder(folder, b',') {
        Ok(dfs) => {
            println!("Parsed {} files", dfs.len());
            for (i, df) in dfs.iter().enumerate() {
                println!("\nFile {}:", i + 1);
                println!("Headers: {:?}", df.get_column_names());
                println!("Number of rows: {}", df.height());
                let preview = df.head(Some(5));
                println!("[csv_polars_cleaner] Preview (first 5 rows):");
                for row_idx in 0..preview.height() {
                    let mut row = Vec::new();
                    for col in preview.get_column_names() {
                        if let Ok(series) = preview.column(col) {
                            match series.get(row_idx) {
                                Ok(polars::prelude::AnyValue::String(inner)) => row.push(inner.to_string()),
                                Ok(polars::prelude::AnyValue::Null) => row.push("null".to_string()),
                                Ok(other) => row.push(format!("{}", other)),
                                Err(_) => row.push("<err>".to_string()),
                            }
                        }
                    }
                    println!("[csv_polars_cleaner] Row {}: {:?}", row_idx + 1, row);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to parse folder: {:?}", e);
        }
    }
}
