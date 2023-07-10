A small sample project to showcase a REST web service with a database using

- [iron - An Extensible, Concurrent Web Framework for Rust](https://github.com/iron/iron)
- [diesel - a Safe, Extensible ORM and Query Builder](https://diesel.rs/)
- [serde - a framework for serializing and deserializing](https://serde.rs/)

# TODO

- Create `diesel migration` scripts for the user table
- Store three default users in the database
- Run migrations at the start of the application
- Create new endpoints /users/1 and /users/all and connect them to the database
- Add field `created_at: NaiveDateTime` (chrono)

# Development

## Requirements

- Rustup >= 1.26.0
- Rust >= 1.70.0
- Diesel CLI >= 2.1.0

## Database migrations: Diesel setup

### Start local database via docker

```bash
docker run -it --rm -p 26257:26257 -p 26258:8080 -e COCKROACH_DATABASE=rust cockroachdb/cockroach start-single-node --insecure
```

### Install Diesel CLI

I used this command:

```bash
cargo install diesel_cli --no-default-features --features "postgres sqlite"
diesel setup --database-url "postgresql://root@172.17.0.1:26257/rust?sslmode=disable"
```

### Thoughts on Diesel

I think it is a pity there is no real Standard for SQL (despite it's name).
In my projects I like to support

- Postgres for production databases
- sqlite when low performance is sufficient
- h2 for integration tests

With Diesel, I would need to have (and maintain) three different versions of the same migration script,
because of the slightly different syntax for each database vendor. :(

I had the same problem at [homecloud](https://gitlab.com/neonews-homecloud/api/). I used Liquibase there to solve this,
and they solved the problem by using their very own file format (see example
[here](https://gitlab.com/neonews-homecloud/api/-/blob/89843abb601e296c6bdfc6cfc53c6d91d6b2096c/resources/de/neonew/homecloud/database/migration/001_users-table.yaml)).

## Start server in development mode

```bash
cargo run
```

Less verbose logging level (see [here](https://docs.rs/env_logger/latest/env_logger/#enabling-logging)):

```bash
RUST_LOG=warn cargo run
```

Run on a different port:

```bash
cargo run -- --port 8000
# Alternative:
# PORT=8000 cargo run
```
