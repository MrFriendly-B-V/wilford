# Wilford's server
This is the backend of the application.

## Build dependencies
- [Rust compiler](https://rust-lang.rog)
- C compiler

## Runtime dependencies
- MySQL / MariaDB
- (Optional) EspoCRM installation

## Configuration
### Environmental variables
```bash
RUST_LOG=<logging directive>
CONFIG_PATH=<path to config.json>
```

### JSON
Refer to the structs [here](./wilford/src/config.rs)

## Running
```
export RUST_LOG=INFO,wilford=TRACE
export CONFIG_PATH=./config.json

cargo run -p wilford
```
The server will expose its API on port `2521`.

## Building
Debug mode:
```
cargo build -p wilford
```
Release mode:
```
cargo build -p wilford --release
```
Docker:
```
docker build -t <image tag> .
```