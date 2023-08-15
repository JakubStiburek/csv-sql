use crate::prelude::*;

pub fn process_csv_file(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open(file_path)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

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

    let mut sql = String::new();

    create_table(&mut sql, &name)?;

    append_inserts(&mut sql, &name, &headers, &records)?;

    let mut output_file = File::create(format!("{}.sql", name))?;
    output_file.write_all(sql.as_bytes())?;

    Ok(())
}

fn create_table<'a>(sql: &'a mut String, name: &'a String) -> Result<&'a mut String, Box<dyn Error>> {
    println!("Creating table {}...", name);

    sql.push_str(&format!("CREATE TABLE {} (", name));

    println!("Table {} created. 🚀", name.on_green());

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
