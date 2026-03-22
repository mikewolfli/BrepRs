# BrepRs Language Bindings

This directory contains language bindings for BrepRs, allowing you to use BrepRs from various programming languages.

## Supported Languages

- **Python** - Using PyO3
- **C/C++** - Using FFI
- **Fortran** - Using C FFI
- **Java** - Using JNI
- **Node.js** - Using NAPI-RS
- **PHP** - Using ext-php-rs

## Building Bindings

### Python

```bash
# Build Python bindings
cargo build --release --features python

# Install maturin for building Python packages
pip install maturin

# Build and install Python package
cd bindings/python
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

### Fortran

Fortran bindings use the C FFI interface. Build the C library first, then compile Fortran code:

```bash
# Build C library
cargo build --release --lib

# Compile Fortran example
gfortran -o example examples/fortran/example.f90 -L target/release -lbreprs -I bindings/cpp
```

### Java

```bash
# Build Java bindings
cargo build --release --features java

# The native library will be in target/release/libbreprs.so (Linux/Mac)
# or target/release/breprs.dll (Windows)
```

### Node.js

```bash
# Build Node.js bindings
cargo build --release --features nodejs

# Install napi-rs CLI
cargo install napi-rs-cli

# Build Node.js package
cd bindings/nodejs
napi build --release
```

### PHP

```bash
# Build PHP extension
cargo build --release --features php

# Install the extension
cp target/release/libbreprs.so $(php-config --extension-dir)/breprs.so
echo "extension=breprs.so" >> $(php --ini | grep "Loaded Configuration" | sed 's/.*=> //')
```

## Examples

Each language has example code in the `examples/` directory:

- `examples/python/example.py` - Python example
- `examples/cpp/example.cpp` - C++ example
- `examples/fortran/example.f90` - Fortran example
- `examples/java/Example.java` - Java example
- `examples/nodejs/example.js` - Node.js example
- `examples/php/example.php` - PHP example

## Running Examples

### Python

```bash
cd examples/python
python example.py
```

### C++

```bash
cd examples/cpp
g++ -o example example.cpp -I../.. -L../../target/release -lbreprs
LD_LIBRARY_PATH=../../target/release ./example
```

### Fortran

```bash
cd examples/fortran
gfortran -o example example.f90 -L../../target/release -lbreprs -I../../bindings/cpp
LD_LIBRARY_PATH=../../target/release ./example
```

### Java

```bash
cd examples/java
javac Example.java
java -Djava.library.path=../../target/release Example
```

### Node.js

```bash
cd examples/nodejs
node example.js
```

### PHP

```bash
cd examples/php
php example.php
```

## API Reference

### Common Functions

All bindings provide the following core functions:

#### i18n_init()
Initialize internationalization with automatic language detection.

#### i18n_set_language(lang_code: string) -> bool
Set the current language. Returns `true` if successful.

#### i18n_current_language() -> string
Get the current language code.

#### i18n_translate(key: string) -> string
Translate a message key to the current language.

#### i18n_available_languages() -> array
Get all available language codes.

### Language Codes

- `en` - English
- `zh-CN` - Simplified Chinese
- `zh-TW` - Traditional Chinese
- `fr` - French
- `de` - German
- `ru` - Russian

### Message Keys

All message keys are defined in the MessageKey enum. See the source code for the complete list.

Common message keys include:

- `ErrorUnknown` - Unknown error
- `ErrorInvalidInput` - Invalid input
- `LabelFile` - File menu label
- `LabelEdit` - Edit menu label
- `OpBooleanFuse` - Boolean fuse operation
- `OpBooleanCut` - Boolean cut operation
- `OpBooleanCommon` - Boolean common operation

## Troubleshooting

### Library Path Issues

If you encounter library loading errors, make sure the library path is set correctly:

- **Linux/Mac**: `export LD_LIBRARY_PATH=target/release`
- **Windows**: Add `target\release` to PATH

### Python Import Errors

If you cannot import the Python module, make sure you've built and installed it:

```bash
cd bindings/python
maturin develop --release
```

### Java UnsatisfiedLinkError

If you get `UnsatisfiedLinkError`, make sure the native library is in the correct location:

```bash
java -Djava.library.path=target/release Example
```

### Node.js Module Not Found

If Node.js cannot find the module, build it first:

```bash
cd bindings/nodejs
napi build --release
```

### PHP Extension Not Loading

If the PHP extension is not loading, check the PHP configuration:

```bash
php -m | grep breprs
```

## Contributing

When adding new functions to the bindings, make sure to:

1. Add the function to all language bindings
2. Update the examples
3. Test the bindings on all supported platforms
4. Update this README

## License

These bindings follow the same license as BrepRs (MIT OR Apache-2.0).
