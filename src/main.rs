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
    pub use std::io::stdout;
    pub use std::process::exit;
    pub use exitcode;
    pub use std::str::FromStr;
    pub use std::path::PathBuf;
}

use prelude::*;


#[derive(Parser, Debug)]
#[command(author = "Jakub Stibůrek", version = "0.3.0", about = "CSV -> SQL table")]
struct Args {
    #[arg(num_args(0..), required = true, help = "Input CSV file(s)")]
    file_paths: Vec<PathBuf>,

    #[arg(short, long, help = "Generate only schema without inserts")]
    schema_only: bool,

    #[arg(short, long, value_names(&["smallint|integer|bigint"]), help = "Generate primary key 'id' column with serial type")]
    primary_key: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let paths = args.file_paths.iter().map(|s| Path::new(s)).collect();
    let mut options: Vec<ConfigOption> = vec![];
    let schema_only = if args.schema_only { Some(ConfigOption::SchemaOnly) } else { None };
    if let Some(primary_key) = args.primary_key {
        match SerialSize::from_str(&primary_key) {
            Ok(size) => options.push(ConfigOption::PrimaryKey(PrimarySerial::new(size))),
            Err(err) => {
                eprintln!("Invalid primary key size: {}", err);
                exit(exitcode::DATAERR);
            }
        }
    }

    if let Some(option) = schema_only {
        options.push(option);
    }


    match validate_file_paths(&paths) {
        Ok(_) => {}
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
            exit(exitcode::DATAERR);
        }
    }

    for file_path in paths {
        match process_csv_file(Config::new(&options, file_path)) {
            Ok(sql) => {
                stdout().write_all(sql.as_bytes())?;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                exit(exitcode::DATAERR);
            }
        };
    }

    Ok(())
}
