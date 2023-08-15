# CSV-SQL
## A simple CLI tool to generate SQL tables from CSV files

- CSV-SQL will generate SQL table creation statements from CSV files, currently with headers and TEXT data types only.

### Usage
#### Generate SQL table creation statements from CSV files
- The name of the CSV file will be used as the name of the table and the file.
```bash
csv-sql [FILE_PATHS]...
```
#### Override default name of the table and the file
```bash
csv-sql [FILE_PATHS]... --names=[name],[name],[name]...
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
- [x] Override default name of the table
- [ ] ... without headers
- [ ] ... with headers and data types
- [ ] ... with headers, data types and primary keys
- [ ] ... with headers, data types, primary keys and constraints
- [ ] ... with headers, data types, primary keys, constraints and foreign keys
- [ ] merge multiple CSV files into one SQL table
