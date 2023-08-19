# CSV-SQL
## A simple CLI tool to generate SQL tables from CSV files

- CSV-SQL will generate SQL table creation statements from CSV files, currently with TEXT data types only.
- CSV-SQL outputs to stdout. You can redirect the output to a file.

### Usage
#### Generate SQL table creation statements from CSV files
- The name of the CSV file will be used as the name of the table.
```bash
csv-sql [FILE_PATHS]...
```

#### Generate only the schema without inserting data
```bash
csv-sql --schema-only [FILE_PATHS]...
```

#### Add a serial primary key
```bash
csv-sql --primary-key=smallint [FILE_PATHS]...
csv-sql --primary-key=integer [FILE_PATHS]...
csv-sql --primary-key=bigint [FILE_PATHS]...
```

#### Merge multiple tables into one
```bash
csv-sql --merge [FILE_PATHS]...
```

#### Redirect output to a file
```bash
csv-sql [FILE_PATHS]... > output.sql
```

#### See all options
```bash
csv-sql --help
```

### Development
Use [just](https://github.com/casey/just) for easy development.
#### See all available commands
```bash
just -l
``` 
#### Build and run
```bash
just dev ~/Dir/file.csv
just dev "--schema-only ~/Dir/file.csv ~/Dir/file2.csv ~/Dir/file3.csv"
```
#### Clear all *.sql files in current directory
```bash
just clear
```

### TODO
- [x] Generate SQL table creation statements from CSV files with headers and TEXT data types
- [x] Output to stdout
- [x] Schema only without inserting data
- [x] merge multiple CSV files into one SQL table
- [ ] Automatic data type detection
- [ ] Custom data types
- [ ] Primary keys
  - [x] Auto increment
  - [ ] Custom
- [ ] Constraints
- [ ] Foreign keys

### Contact
- [Email: jakub@stiburek.dev](mailto:jakub@stiburek.dev)
- [LI: Jakub Stiburek](https://www.linkedin.com/in/jakubstiburekdev/)
