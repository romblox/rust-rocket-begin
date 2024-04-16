## SQlite  
SQLite is a C-language library that implements a small, fast, self-contained, high-reliability, full-featured,  
https://www.sqlite.org  

### Install sqlite3

```shell
sudo apt update
sudo apt install sqlite3 libsqlite3-dev -y
```

## Diesel  
`Diesel` is a Safe, Extensible ORM and Query Builder for Rust  
https://diesel.rs/  


### Getting Started with Diesel  
https://diesel.rs/guides/getting-started.html  

### Install diesel_cli

```shell
cargo install diesel_cli --no-default-features --features sqlite
```
By default, diesel CLI depends on the following client libraries:
- `libpq` for the PostgreSQL backend  
- `libmysqlclient` for the Mysql backend  
- `libsqlite3` for the SQLite backend  

### Init database with diesel  
```shell
diesel setup --database-url ./database.sqlite
```

### Create table in database using diesel migration  
```shell
diesel migration generate create_rustaceans
```

### List migrations  
```shell
diesel migration list --database-url=database.sqlite
```

### Run migration  
```shell
diesel migration run --database-url=database.sqlite
```

### Revert migration  
```shell
diesel migration revert --database-url=database.sqlite
```