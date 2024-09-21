# This is a simple key-value store written in Rust. It is designed to be a drop-in replacement for Redis.

## Features

- [x] Set and get values
- [x] Delete values
- [x] Save and load from file
- [x] Check the database integrity by validating the hash of the data

## Implementation

The store is implemented as a Rust `HashMap` that is stored in a `BTreeMap`. This allows for fast lookups and inserts.

The store is designed to be a drop-in replacement for Redis. It is not designed to be a high-performance store. It is designed to be a simple store that can be used to store and retrieve values.

The store is designed to be a simple store that can be used to store and retrieve values.

In the future it will change to a B+Tree to allow for range queries on keys.

We want to persist the store to disk to allow for long term storage of the store.

## Usage

```rust
use pixiekv::PixieKV;

let mut store = PixieKV::default();

store.insert("key", "value");

let value = store.get("key");

store.remove("key");
```

## Run tests

```sh
cargo test
```

## License

MIT