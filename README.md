A small sample project to showcase a REST web service with a database using

- [iron - An Extensible, Concurrent Web Framework for Rust](https://github.com/iron/iron)
- [diesel - a Safe, Extensible ORM and Query Builder](https://diesel.rs/)
- [serde - a framework for serializing and deserializing](https://serde.rs/)

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
# PORT=1234 cargo run
```
