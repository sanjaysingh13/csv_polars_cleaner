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
            }
        }
        Err(e) => {
            eprintln!("Failed to parse folder: {:?}", e);
        }
    }
}
