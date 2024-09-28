# PixieKV: A Simple Key-Value Store in Rust

PixieKV is a lightweight, no_std compatible key-value store written in Rust. It is designed to be a simple and efficient storage solution for embedded systems and other resource-constrained environments.

## Features

- [x] Set and get values
- [x] Delete values
- [x] Save and load from file
- [x] No_std compatible
- [x] Persistent storage using LittleFS
- [x] Generic value types (supports any type that implements Serialize and Deserialize)
- [x] Fixed-size storage with compile-time checks
- [ ] Check the database integrity by validating the hash of the data

## Implementation

PixieKV is implemented using a `heapless::FnvIndexMap` as the underlying data structure. This allows for efficient lookups and inserts while maintaining a fixed-size storage that's suitable for embedded systems.

The key-value store is designed to be simple, efficient, and easy to use in no_std environments. It uses the LittleFS file system for persistent storage, allowing data to be saved and loaded from flash memory or other storage mediums.

## Usage

```rust
use pixiekv::PixieKV;

let mut store = PixieKV::default();

store.insert("key", "value");

let value = store.get("key");

store.remove("key");

store.save_to_file("database.db");

let loaded_store = PixieKV::load_from_file("database.db");
```

## Run tests

```sh
cargo test
```

## Run examples

```sh
cargo run --example std_example
```

## Run emulated tests

```sh
cargo run --target thumbv7m-none-eabi --release
```

## License

MIT
