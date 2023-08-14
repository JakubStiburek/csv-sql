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

### TODO
- [x] Generate SQL table creation statements from CSV files with headers and TEXT data types
- [ ] ... without headers
- [ ] ... with headers and data types
- [ ] ... with headers, data types and primary keys
- [ ] ... with headers, data types, primary keys and constraints
- [ ] ... with headers, data types, primary keys, constraints and foreign keys
