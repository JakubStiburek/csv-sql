use crate::prelude::*;

pub fn process_csv_file(file_path: &Path) -> Result<(), Box<dyn Error>> {
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

    match create_table(&name, &headers, &records) {
        Ok(_) => {}
        Err(err) => {
            return Err(err);
        }
    }

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
