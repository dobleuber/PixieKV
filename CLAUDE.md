# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

PixieKV is a lightweight, no_std compatible key-value store written in Rust for embedded systems. It uses heapless data structures and LittleFS for persistent storage, making it suitable for resource-constrained environments.

## Development Commands

### Testing
- `cargo test` - Run all unit tests (includes both lib.rs and domain module tests)
- `cargo test --lib` - Run only library tests
- `cargo test --test integration` - Run integration tests (if any)

### Building
- `cargo build` - Build in debug mode
- `cargo build --release` - Build optimized release version
- `cargo build --target thumbv7m-none-eabi --release` - Build for ARM Cortex-M target (embedded)

### Examples
- `cargo run --example std_example` - Run the standard environment example

### Emulated Testing
- `cargo run --target thumbv7m-none-eabi --release` - Run on emulated ARM Cortex-M target using QEMU

## Architecture

### Core Components

**PixieKV Trait** (`src/domain/pixie_kv.rs`): Defines the key-value store interface with `insert`, `get`, and `remove` operations.

**PixieKVStore** (`src/domain/pixie_kv_store.rs`): Main implementation using `heapless::FnvIndexMap` for fixed-size, no-heap storage. Supports generic value types that implement `Serialize` and `Deserialize`.

**Storage Layer** (`src/domain/storage.rs`): Handles LittleFS integration for persistent storage operations.

**Persistent Module** (`src/domain/persistent.rs`): Manages file I/O operations and serialization/deserialization using postcard format. Handles integrity hash storage as separate .hash files.

### Key Constraints

- **Fixed Size**: Storage is limited by `MAX_SIZE` constant (defined in `src/domain/constants.rs`)
- **Key Length**: Keys are limited by `MAX_KEY_LEN` constant
- **No Heap**: Uses heapless data structures suitable for embedded environments
- **Serialization**: Values must implement `Serialize` and `Deserialize` traits

### Error Handling

The codebase uses `Result<T, &'static str>` for error handling with specific error messages:
- "Key too long" - When key exceeds `MAX_KEY_LEN`
- "Database is full" - When storage exceeds `MAX_SIZE`
- File I/O errors are handled through the `Error` enum in the persistent module
- `IntegrityError` - When data integrity validation fails during load operations

### Testing Strategy

Tests are organized at multiple levels:
- Unit tests in `src/lib.rs` for basic operations
- Comprehensive tests in `src/domain/pixie_kv_store.rs` including edge cases
- LittleFS integration tests for persistence functionality
- Size constraint validation tests

## Data Integrity Features

### Integrity Validation
The codebase includes CRC32-based integrity validation to detect data corruption:

**Core Methods:**
- `calculate_hash()` - Computes CRC32 of stored data
- `verify_integrity()` - Validates current data against stored hash
- `update_integrity_hash()` - Recalculates and updates hash after modifications
- `get_integrity_hash()` - Returns current integrity hash value

**Enhanced Operations:**
- `insert_with_integrity()` - Insert with automatic hash update
- `remove_with_integrity()` - Remove with automatic hash update
- `load_from_file_with_verification()` - Load with integrity verification

**File Storage:**
- Data is stored in main file (e.g., "database.db")
- Integrity hash stored in companion file (e.g., "database.db.hash")
- Both files are required for successful load operations

## Important Notes

- This is a no_std crate that can optionally work in std environments for testing
- All persistence operations require LittleFS filesystem formatting before first use
- The codebase is designed for embedded ARM Cortex-M targets but includes std compatibility for development
- For integrity-aware operations, use `*_with_integrity` methods or enable verification during load
- Regular insert/remove operations don't automatically update integrity hash for performance
- All save operations create both data and hash files; both are required for loading