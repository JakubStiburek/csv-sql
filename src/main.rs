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

    #[arg(short, long)]
    names: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let paths = args.file_paths.iter().map(|s| Path::new(s)).collect();
    let names = match &args.names {
        Some(arg_names) => arg_names.split(",").collect::<Vec<&str>>(),
        None => vec![],
    };

    match validate_file_paths(&paths) {
        Ok(ok_paths) => {
            eprintln!("These files will be processed:");
            for path in ok_paths {
                eprintln!("{}", path);
            }
        }
        Err(res) => {
            eprintln!("Invalid input:");
            for error in res.1 {
                eprintln!("{}", error);
            }
            for ok_path in res.0 {
                eprintln!("{}", "-".repeat(50));
                eprintln!("Valid input:");
                eprintln!("{}", ok_path);
            }
        }
    }

    for (i, file_path) in paths.iter().enumerate() {
        let name = match &names.get(i) {
            Some(name) => Some(name.to_string()),
            None => None,
        };

        match process_csv_file(Config::new(Mode::Headers, name, file_path)) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        };
    }

    return Ok(());
}
