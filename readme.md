# CSV-SQL
## A simple CLI tool to generate SQL tables from CSV files

- CSV-SQL will generate SQL table creation statements from CSV files, currently with headers and TEXT data types only.
- The name of the CSV file will be used as the name of the table and the name of resulting sql type.

### Usage
#### Generate SQL table creation statements from CSV files
```bash
csv-sql [FILE_PATHS]...
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
just run ~/Dir/file.csv
just run "~/Dir/file.csv ~/Dir/file2.csv ~/Dir/file3.csv"
```
#### Clear the all *.sql files in current directory
```bash
just clear
```

### TODO
- [x] Generate SQL table creation statements from CSV files with headers and TEXT data types
- [ ] ... without headers
- [ ] ... with headers and data types
- [ ] ... with headers, data types and primary keys
- [ ] ... with headers, data types, primary keys and constraints
- [ ] ... with headers, data types, primary keys, constraints and foreign keys
