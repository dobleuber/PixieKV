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
- [x] Database integrity validation using CRC32 hashing

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

## Prerequisites

### Installing QEMU

For running emulated tests on ARM Cortex-M targets, you need to install QEMU:

#### **Ubuntu/Debian Linux**
```bash
sudo apt update
sudo apt install qemu-system-arm
```

#### **macOS**
Using Homebrew:
```bash
brew install qemu
```

Using MacPorts:
```bash
sudo port install qemu
```

#### **Windows**
1. Download QEMU from the official website: https://www.qemu.org/download/#windows
2. Run the installer and follow the installation wizard
3. Add QEMU to your system PATH:
   - Open "Environment Variables" in System Properties
   - Add the QEMU installation directory (usually `C:\Program Files\qemu`) to the PATH variable

Alternatively, using Chocolatey:
```powershell
choco install qemu
```

Or using Scoop:
```powershell
scoop install qemu
```

### ARM GNU Toolchain
Install the ARM cross-compilation toolchain:

#### **Ubuntu/Debian Linux**
```bash
sudo apt update
sudo apt install gcc-arm-none-eabi
```

#### **macOS**
Using Homebrew:
```bash
brew install arm-none-eabi-gcc
```

#### **Windows**
1. Download ARM GNU Toolchain from: https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads
2. Install the "arm-none-eabi" variant for your Windows version
3. Add the toolchain `bin` directory to your system PATH (usually `C:\Program Files (x86)\Arm GNU Toolchain arm-none-eabi\<version>\bin`)

Alternatively, using Chocolatey:
```powershell
choco install gcc-arm-embedded
```

### Rust ARM Target
Install the ARM Cortex-M target for Rust:
```bash
rustup target add thumbv7m-none-eabi
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
