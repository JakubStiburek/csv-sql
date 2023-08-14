use clap::Parser;
use std::{fs, path::Path, error::Error};
use std::fs::File;
use std::io::{Read, Write};
use csv::ReaderBuilder;
use colored::*;

#[derive(Parser, Debug)]
#[command(author = "Jakub Stibůrek", version = "0.1.0", about = "CSV -> SQL table")]
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

fn process_csv_file(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(contents.as_bytes());

    let name = file_path.file_stem().unwrap().to_str().unwrap().to_string();
    let headers = csv_reader.headers()?.iter().map(|s| s.to_string()).collect();
    let mut records: Vec<Vec<String>> = vec![];

    for record in csv_reader.records() {
        let record = record?;
        let record_strings: Vec<String> = record.iter().map(|field| field.to_string()).collect();
        records.push(record_strings);
    }

    create_table(&name, &headers, &records)?;

    Ok(())
}

fn create_table(name: &String, headers: &Vec<String>, records: &Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    println!("Creating table {}...", name);

    let mut file = File::create(format!("{}.sql", name))?;

    let mut sql = format!("CREATE TABLE {} (", name);

    let columns = headers.iter().map(|s| format!("{} TEXT", s)).collect::<Vec<String>>().join(", ");

    sql.push_str(&columns);

    sql.push_str(");");

    for record in records {
        let mut sql_record = format!("INSERT INTO {} VALUES (", name);

        let values = record.iter().map(|s| format!("'{}'", s)).collect::<Vec<String>>().join(", ");

        sql_record.push_str(&values);

        sql_record.push_str(");");

        sql.push_str("\n");
        sql.push_str(&sql_record);
    }

    file.write_all(sql.as_bytes())?;

    println!("Table {} created. 🚀", name.on_green());

    Ok(())
}
