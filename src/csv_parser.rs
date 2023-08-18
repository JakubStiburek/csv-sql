use crate::prelude::*;

#[derive(PartialEq)]
pub enum ConfigOption {
    SchemaOnly
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
    let mut input_file = File::open(config.file_path)?;
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
    }

    let mut sql = String::new();

    create_table(&mut sql, &config.name, &headers)?;

    if config.options.contains(&ConfigOption::SchemaOnly) {
        return Ok(sql);
    }

    append_inserts(&mut sql, &config.name, &records)?;

    Ok(sql)
}

fn create_table<'a>(sql: &'a mut String, name: &'a String, headers: &'a Vec<String>) -> Result<&'a mut String, Box<dyn Error>> {

    sql.push_str(&format!("CREATE TABLE {} (", name));

    let columns = headers.iter().map(|s| format!("{} TEXT", s)).collect::<Vec<String>>().join(", ");

    sql.push_str(&columns);

    sql.push_str(");");

    Ok(sql)
}

fn append_inserts<'a>(sql: &'a mut String, name: &'a String, records: &'a Vec<Vec<String>>) -> Result<&'a mut String, Box<dyn Error>> {
    for record in records {
        let mut sql_record = format!("INSERT INTO {} VALUES (", name);

        let values = record.iter().map(|s| format!("'{}'", s)).collect::<Vec<String>>().join(", ");

        sql_record.push_str(&values);

        sql_record.push_str(");");

        sql.push_str("\n");

        sql.push_str(&sql_record);
    }

    Ok(sql)
}
