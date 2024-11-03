# Configuration


## Default config file
```json
{{#include ../../../sample_config.json}}
```

## Environmental variables
```
CONFIG_PATH=<path to config.json>
```

## Available options
The following Rust structs define the layout of the configuration.
An example of how this translates to JSON can be found in the [sample config](#default-config-file)
```rust,noplayground
{{#include ../../../server/wilford/src/config.rs:config}}
```