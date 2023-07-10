A small sample project to showcase a REST web service with a database using

- [iron - An Extensible, Concurrent Web Framework for Rust](https://github.com/iron/iron)
- [diesel - a Safe, Extensible ORM and Query Builder](https://diesel.rs/)
- [serde - a framework for serializing and deserializing](https://serde.rs/)

# Development

## Requirements

- Rustup >= 1.26.0
- Rust >= 1.70.0

## Start server in development mode

```bash
cargo run
```

Less verbose logging level (see [here](https://docs.rs/env_logger/latest/env_logger/#enabling-logging)):

```bash
RUST_LOG=warn cargo run
```
