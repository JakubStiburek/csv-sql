mod csv_parser;

mod prelude {
    pub use clap::Parser;
    pub use std::{fs, path::Path, error::Error};
    pub use std::fs::File;
    pub use std::io::{Read, Write};
    pub use csv::ReaderBuilder;
    pub use colored::*;
    pub use crate::csv_parser::*;
}

use prelude::*;


#[derive(Parser, Debug)]
#[command(author = "Jakub Stibůrek", version = "0.1.2", about = "CSV -> SQL table")]
struct Args {
    #[arg(num_args(0..), required = true)]
    file_paths: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match validate_args(&args) {
        Ok(ok_paths) => {
            println!("These files will be processed:");
            for path in ok_paths {
                println!("{}", path);
            }
        }
        Err(res) => {
            println!("Invalid input:");
            for error in res.1 {
                println!("{}", error);
            }
            for ok_path in res.0 {
                println!("{}", "-".repeat(50));
                println!("Valid input:");
                println!("{}", ok_path);
            }
        }
    }

    for file_path in &args.file_paths {
        process_csv_file(Path::new(file_path))?;
    }

    return Ok(());
}

fn is_csv_file(file_path: &Path) -> bool {
    file_path.extension().map_or(false, |ext| ext == "csv")
}


fn validate_args(args: &Args) -> Result<Vec<String>, (Vec<String>, Vec<String>)> {
    let mut ok_paths: Vec<String> = vec![];
    let mut errors: Vec<String> = vec![];

    for path in &args.file_paths {
        if let Ok(metadata) = fs::metadata(path) {
            if metadata.is_file() {
                if is_csv_file(Path::new(path)) {
                    ok_paths.push(format!("{}", path.on_green()));
                } else {
                    errors.push(format!("{} - not a CSV file", path.on_red()));
                }
            } else {
                errors.push(format!("{} - not a file", path.on_red()));
            }
        } else {
            errors.push(format!("{} - invalid file path", path.on_red()));
        }
    }

    if errors.len() > 0 {
        return Err((ok_paths, errors));
    }

    return Ok(ok_paths);
}


