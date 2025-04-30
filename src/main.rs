use csv_polars_cleaner::parse_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <csv_file_path>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];
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