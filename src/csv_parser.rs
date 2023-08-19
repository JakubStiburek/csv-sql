use crate::prelude::*;

#[derive(PartialEq, Debug, Parser)]
pub enum SerialSize {
    SmallInt,
    Integer,
    BigInt,
}

impl FromStr for SerialSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "smallint" => Ok(SerialSize::SmallInt),
            "integer" => Ok(SerialSize::Integer),
            "bigint" => Ok(SerialSize::BigInt),
            _ => Err(format!("Invalid serial size: {}", s)),
        }
    }
}

#[derive(PartialEq)]
pub struct PrimarySerial {
    size: SerialSize,
}

impl PrimarySerial {
    pub fn new(size: SerialSize) -> PrimarySerial {
        PrimarySerial { size }
    }
}

#[derive(PartialEq)]
pub enum ConfigOption {
    SchemaOnly,
    PrimaryKey(PrimarySerial),
    Merge,
}

struct ReadFile {
    headers: Vec<String>,
    records: Vec<Vec<String>>,
}

impl ReadFile {
    fn new(headers: Vec<String>, records: Vec<Vec<String>>) -> ReadFile {
        ReadFile { headers, records }
    }
}

pub struct Config<'a> {
    pub options: &'a Vec<ConfigOption>,
    pub name: String,
    pub file_path: &'a Path,
}

impl Config<'_> {
    pub fn new<'a>(options: &'a Vec<ConfigOption>, file_path: &'a Path) -> Config<'a> {
        Config {
            options,
            name: file_path.file_stem().unwrap().to_str().unwrap().to_string(),
            file_path,
        }
    }
}

pub fn process_csv_file(config: Config) -> Result<String, Box<dyn Error>> {
    let ReadFile { headers, records } = read_file(config.file_path)?;

    let primary_serial = config.options.iter().find_map(|option| {
        match option {
            ConfigOption::PrimaryKey(serial) => Some(serial),
            _ => None,
        }
    });

    match create_table(&config.name, &headers, primary_serial) {
        Ok(sql) => {
            if config.options.contains(&ConfigOption::SchemaOnly) {
                return Ok(sql);
            }

            let inserts = append_inserts(&config.name, &records, &headers)?;

            Ok(sql + &inserts)
        }
        Err(err) => Err(err),
    }
}

pub fn process_csv_files(config: Config, file_paths: Vec<&Path>) -> Result<String, Box<dyn Error>> {
    let primary_serial = config.options.iter().find_map(|option| {
        match option {
            ConfigOption::PrimaryKey(serial) => Some(serial),
            _ => None,
        }
    });
    let mut all_headers: Vec<String> = vec![];
    let mut all_records: Vec<Vec<Vec<String>>> = vec![];

    for path in file_paths {
        let ReadFile { headers, records } = read_file(path)?;

        all_headers.extend(headers);
        all_records.push(records);
    }

    match create_table(&config.name, &all_headers, primary_serial){
        Ok(sql) => {
            if config.options.contains(&ConfigOption::SchemaOnly) {
                return Ok(sql);
            }

            let merged_records = merge_records(all_records);

            let inserts = append_inserts(&config.name, &merged_records, &all_headers)?;

            Ok(sql + &inserts)
        },
        Err(err) => Err(err),

    }
}

fn merge_records(records: Vec<Vec<Vec<String>>>) -> Vec<Vec<String>> {
    let mut merged_records: Vec<Vec<String>> = Vec::new();

    if let Some(max_len) = records.iter().map(|inner| inner.len()).max() {
        for i in 0..max_len {
            let mut merged_row: Vec<String> = Vec::new();

            for inner in &records {
                let row = if i < inner.len() {
                    inner[i].clone()
                } else {
                    vec![String::new(); inner[0].len()]
                };

                merged_row.extend(row);
            }

            merged_records.push(merged_row);
        }
    }

    merged_records
}

fn create_table<'a>(name: &'a String, headers: &'a Vec<String>, primary_serial: Option<&'a PrimarySerial>) -> Result<String, Box<dyn Error>> {
    let mut sql = String::new();

    sql.push_str(&format!("CREATE TABLE {} (", name));

    if let Some(PrimarySerial { size }) = primary_serial {
        sql.push_str(&format!("id {} PRIMARY KEY, ", match size {
            SerialSize::SmallInt => "SMALLSERIAL",
            SerialSize::Integer => "SERIAL",
            SerialSize::BigInt => "BIGSERIAL",
        }));
    }

    let columns = headers.iter().map(|s| format!("{} TEXT", s)).collect::<Vec<String>>().join(", ");

    sql.push_str(&columns);

    sql.push_str(");");

    Ok(sql)
}

fn append_inserts<'a>(name: &'a String, records: &'a Vec<Vec<String>>, headers: &'a Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut sql = String::new();

    for record in records {
        let columns = headers.iter().map(|s| format!("{}", s)).collect::<Vec<String>>().join(", ");

        let mut sql_record = format!("INSERT INTO {} ({}) VALUES (", name, columns);

        let values = record.iter().map(|s| format!("'{}'", s)).collect::<Vec<String>>().join(", ");

        sql_record.push_str(&values);

        sql_record.push_str(");");

        sql.push_str("\n");

        sql.push_str(&sql_record);
    }

    Ok(sql)
}

fn read_file(file_path: &Path) -> Result<ReadFile, Box<dyn Error>> {
    let mut input_file = File::open(file_path)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(contents.as_bytes());

    let headers = csv_reader.headers()?.iter().map(|s| s.to_string()).collect();
    let mut records: Vec<Vec<String>> = vec![];

    for record in csv_reader.records() {
        let record = record?;
        let record_strings: Vec<String> = record.iter().map(|field| field.to_string()).collect();
        records.push(record_strings);
    };

    Ok(ReadFile::new(headers, records))
}
