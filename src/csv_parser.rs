use crate::prelude::*;

pub enum Mode {
    NoHeaders,
    Headers,
}

pub struct Config<'a> {
    pub mode: Mode,
    pub name: String,
    pub file_path: &'a Path,
}

impl Config<'_> {
    pub fn new(mode: Mode, name: Option<String>, file_path: &Path) -> Config {
        let name = match name {
            Some(name) => name,
            None => file_path.file_stem().unwrap().to_str().unwrap().to_string(),
        };

        Config {
            mode,
            name,
            file_path,
        }
    }
}

pub fn process_csv_file(config: Config) -> Result<(), Box<dyn Error>> {
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

    println!("Creating table {}...", &config.name);

    create_table(&mut sql, &config.name)?;

    append_inserts(&mut sql, &config.name, &headers, &records)?;

    println!("Table {} created. 🚀", &config.name.on_green());

    let mut output_file = File::create(format!("{}.sql", config.name))?;
    output_file.write_all(sql.as_bytes())?;

    Ok(())
}

fn create_table<'a>(sql: &'a mut String, name: &'a String) -> Result<&'a mut String, Box<dyn Error>> {

    sql.push_str(&format!("CREATE TABLE {} (", name));

    sql.push_str(");");

    Ok(sql)
}

fn append_inserts<'a>(sql: &'a mut String, name: &'a String, headers: &'a Vec<String>, records: &'a Vec<Vec<String>>) -> Result<&'a mut String, Box<dyn Error>> {
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

    Ok(sql)
}
