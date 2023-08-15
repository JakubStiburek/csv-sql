mod csv_parser;
mod args_validation;

mod prelude {
    pub use clap::Parser;
    pub use std::{fs, path::Path, error::Error};
    pub use std::fs::File;
    pub use std::io::{Read, Write};
    pub use csv::ReaderBuilder;
    pub use colored::*;
    pub use crate::csv_parser::*;
    pub use crate::args_validation::*;
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
    let paths = args.file_paths.iter().map(|s| Path::new(s)).collect();

    match validate_file_paths(paths) {
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
