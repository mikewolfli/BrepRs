# Installation

This section provides detailed instructions for installing BrepRs and its dependencies.

## Rust Installation

### Prerequisites

Before installing BrepRs, you need to have Rust installed on your system. If you don't have Rust installed, follow these steps:

1. **Install Rust** using rustup:
   
   ```bash
   # Linux/macOS
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Windows
   # Download and run rustup-init.exe from https://rustup.rs/
   ```

2. **Verify the installation**:
   
   ```bash
   rustc --version
   cargo --version
   ```

### Installing BrepRs

#### From Crates.io

To use BrepRs in your Rust project, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
breprs = "0.1.0"

# Optional features
[features]
default = ["serde", "rayon"]
serde = ["dep:serde"]
rayon = ["dep:rayon"]
```

#### From Source

To install BrepRs from source:

1. **Clone the repository**:
   
   ```bash
   git clone https://github.com/mikewolfli/breprs.git
   cd breprs
   ```

2. **Build the project**:
   
   ```bash
   # Build in debug mode
   cargo build
   
   # Build in release mode (recommended for production)
   cargo build --release
   ```

3. **Run tests**:
   
   ```bash
   cargo test
   ```

4. **Install the library**:
   
   ```bash
   cargo install --path .
   ```

## Language Bindings

BrepRs provides bindings for multiple programming languages. See the [Language Bindings](../language-bindings.md) section for installation instructions for each language.

### Python

```bash
# Build Python bindings
cd bindings/python
pip install maturin
maturin develop --release
```

### C/C++

```bash
# Build C library
cargo build --release --lib

# Generate C header
cargo install cbindgen
cbindgen --crate breprs --output breprs.h
```

### Java

```bash
# Build Java bindings
cargo build --release --features java
```

### Node.js

```bash
# Build Node.js bindings
cd bindings/nodejs
npm install
npm run build
```

### PHP

```bash
# Build PHP extension
cd bindings/php
cargo build --release
```

## Dependencies

BrepRs has the following dependencies:

| Dependency | Purpose | Optional |
|------------|---------|----------|
| `serde` | Serialization/deserialization | Yes |
| `rayon` | Parallel processing | Yes |
| `wasm-bindgen` | WebAssembly support | Yes |
| `serde_json` | JSON support | Yes |
| `bincode` | Binary serialization | Yes |
| `chrono` | Time handling | No |
| `uuid` | UUID generation | No |
| `zip` | ZIP file support | No |
| `quick-xml` | XML parsing | No |
| `log` | Logging | No |
| `tempfile` | Temporary files | No |
| `libloading` | Dynamic library loading | No |

## Platform Support

BrepRs supports the following platforms:

- **Linux** - x86_64, ARM64
- **macOS** - x86_64, ARM64
- **Windows** - x86_64
- **Web** - WebAssembly

## Troubleshooting

### Common Issues

1. **Missing dependencies**:
   
   If you encounter missing dependencies, install them using your system's package manager:
   
   ```bash
   # Ubuntu/Debian
   sudo apt-get install build-essential libssl-dev
   
   # macOS
   brew install openssl
   
   # Windows
   # Install Visual Studio Build Tools
   ```

2. **Build errors**:
   
   If you encounter build errors, make sure you're using the latest version of Rust:
   
   ```bash
   rustup update
   ```

3. **Linker errors**:
   
   If you encounter linker errors, check your system's linker configuration and make sure all dependencies are installed.

### Getting Help

If you encounter issues during installation, please:

1. Check the [Troubleshooting](../troubleshooting.md) section
2. Search the [GitHub issues](https://github.com/mikewolfli/breprs/issues)
3. Ask for help in the [Discussions](https://github.com/mikewolfli/breprs/discussions)
