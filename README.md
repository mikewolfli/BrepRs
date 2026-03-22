# BrepRs

[![Build Status](https://img.shields.io/github/actions/workflow/status/mikewolfli/BrepRs/rust.yml?branch=main)](https://github.com/mikewolfli/BrepRs/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/breprs.svg)](https://crates.io/crates/breprs)

BrepRs is a Rust implementation of a Boundary Representation (BRep) solid modeling library. It provides a comprehensive set of tools for creating, manipulating, and analyzing 3D geometric models with multi-language support and cross-platform compatibility.

## License

This project is dual-licensed under your choice of either:
- **MIT License** - See [LICENSE-MIT](LICENSE-MIT)
- **Apache License 2.0** - See [LICENSE-APACHE](LICENSE-APACHE)

You may choose to use this software under either license. Both are OSI-approved and compatible with most open source projects.

## Features

### Core Functionality
- **Topological Kernel**: Implementation of BRep data structure with support for vertices, edges, wires, faces, shells, solids, compounds, and compsolids
- **Modeling Algorithms**: Primitive creation, shape construction, boolean operations, fillet/chamfer, and offset operations
- **Data Exchange**: Support for STL, STEP, and IGES file formats
- **Foundation Types**: Basic numeric types, string handling, handle and reference counting, exception handling, and memory management
- **Internationalization**: Multi-language support with hot-reload capability

### Language Bindings
- **Python**: Using PyO3
- **C/C++**: Using FFI
- **Fortran**: Using C FFI
- **Java**: Using JNI
- **Node.js**: Using NAPI-RS
- **PHP**: Using ext-php-rs

### Technical Highlights
- **Rust-Based**: Leverages Rust's safety, performance, and concurrency features
- **Modular Architecture**: Clean, modular design with well-defined interfaces
- **Comprehensive Testing**: Extensive unit tests for all modules
- **Cross-Platform**: Supports Linux, macOS, Windows, and WebAssembly
- **WebAssembly Support**: Can be used in browser applications

## Project Structure

```
/
├── src/                # Source code
│   ├── foundation/     # Foundation types and utilities
│   ├── topology/       # Topological kernel
│   ├── geometry/       # Geometric primitives
│   ├── modeling/       # Modeling algorithms
│   ├── data_exchange/  # File format support
│   ├── i18n/           # Internationalization support
│   │   ├── hot_reload.rs # Hot-reload for translations
│   │   └── mod.rs      # I18n module
│   ├── mesh/           # Mesh generation
│   ├── visualization/  # Visualization
│   ├── application/    # Application framework
│   └── lib.rs          # Library entry point
├── bindings/           # Language bindings
│   ├── python/         # Python bindings
│   ├── cpp/            # C/C++ bindings
│   ├── fortran/        # Fortran bindings
│   ├── java/           # Java bindings
│   ├── nodejs/         # Node.js bindings
│   ├── php/            # PHP bindings
│   └── README.md       # Bindings documentation
├── examples/           # Example code
│   ├── python/         # Python examples
│   ├── cpp/            # C/C++ examples
│   ├── fortran/        # Fortran examples
│   ├── java/           # Java examples
│   ├── nodejs/         # Node.js examples
│   └── php/            # PHP examples
├── translations/       # Translation files
│   ├── en.json         # English translations
│   ├── zh-CN.json      # Simplified Chinese translations
│   ├── zh-TW.json      # Traditional Chinese translations
│   ├── fr.json         # French translations
│   ├── de.json         # German translations
│   └── ru.json         # Russian translations
├── docs/               # Documentation
│   ├── book.toml       # mdbook configuration
│   ├── SUMMARY.md      # Documentation summary
│   └── src/            # Documentation source files
│       ├── en/         # English documentation
│       ├── zh-CN/      # Simplified Chinese documentation
│       ├── zh-TW/      # Traditional Chinese documentation
│       ├── fr/         # French documentation
│       ├── de/         # German documentation
│       └── ru/         # Russian documentation
├── build-docs.sh       # Documentation build script
├── Cargo.toml          # Rust project configuration
└── README.md           # This file
```

## Getting Started

### Prerequisites
- Rust 1.70+ (stable)
- Cargo (Rust package manager)

### Installation

Add BrepRs to your `Cargo.toml`:

```toml
[dependencies]
breprs = "0.6.0-alpha"
```

### Basic Usage

```rust
use breprs::topology::*;
use breprs::modeling::primitive_creation::PrimitiveCreator;
use breprs::data_exchange::stl::StlWriter;
use breprs::i18n::{I18n, Language};

fn main() {
    // Initialize internationalization
    I18n::init();
    I18n::set_language(Language::English);
    
    // Create a box primitive
    let creator = PrimitiveCreator::new();
    let box_shape = creator.make_box(10.0, 10.0, 10.0);
    
    // Create a sphere primitive
    let sphere_shape = creator.make_sphere(5.0);
    
    // Write the shapes to STL files
    let stl_writer = StlWriter::new("box.stl");
    stl_writer.write(&box_shape).unwrap();
    
    let stl_writer = StlWriter::new("sphere.stl");
    stl_writer.write(&sphere_shape).unwrap();
    
    println!("{}", I18n::tr(breprs::i18n::MessageKey::OpPrimitiveCreated));
}
```

## Advanced Usage

### Boolean Operations

```rust
use breprs::modeling::boolean_operations::BooleanOperations;

// Create two shapes
let box_shape = creator.make_box(10.0, 10.0, 10.0);
let sphere_shape = creator.make_sphere(7.0);

// Perform boolean operations
let boolean = BooleanOperations::new();
let fused_shape = boolean.fuse(&box_shape, &sphere_shape);
let cut_shape = boolean.cut(&box_shape, &sphere_shape);
let common_shape = boolean.common(&box_shape, &sphere_shape);
```

### Fillet and Chamfer

```rust
use breprs::modeling::fillet_chamfer::FilletChamfer;

// Create a box
let box_shape = creator.make_box(10.0, 10.0, 10.0);

// Create fillet chamfer tool
let fillet_chamfer = FilletChamfer::new();

// Apply fillet to edges
let filleted_shape = fillet_chamfer.fillet_edges(&box_shape, 1.0);

// Apply chamfer to edges
let chamfered_shape = fillet_chamfer.chamfer_edges(&box_shape, 1.0, 1.0);
```

### Offset Operations

```rust
use breprs::modeling::offset_operations::OffsetOperations;

// Create a face
let face = creator.make_face_from_plane(10.0, 10.0);

// Create offset operations tool
let offset = OffsetOperations::new();

// Offset the face
let offset_face = offset.offset_face(&face, 1.0);

// Create a thick solid
let thick_solid = offset.make_thick_solid_from_face(&face, 1.0, 0.0);
```

### Data Exchange

```rust
use breprs::data_exchange::stl::StlReader;
use breprs::data_exchange::step::StepWriter;
use breprs::data_exchange::iges::IgesWriter;

// Read STL file
let stl_reader = StlReader::new("input.stl");
let shape = stl_reader.read().unwrap();

// Write to STEP file
let step_writer = StepWriter::new("output.step");
step_writer.write(&shape).unwrap();

// Write to IGES file
let iges_writer = IgesWriter::new("output.iges");
iges_writer.write(&shape).unwrap();
```

### Using Language Bindings

#### Python

```python
from breprs_python import I18n, Point, make_box, make_sphere

# Initialize i18n
I18n.init()

# Set language to English
I18n.set_language("en")

# Create a box
origin = Point(0.0, 0.0, 0.0)
box = make_box(1.0, 1.0, 1.0, origin)
print(f"Box created: {box}")
```

#### Node.js

```javascript
const breprs = require('breprs');

// Initialize i18n
breprs.i18nInit();

// Set language to English
breprs.i18nSetLanguage('en');

// Translate a message
console.log('Error message:', breprs.i18nTranslate('ErrorUnknown'));
```

## Documentation

BrepRs provides comprehensive documentation in multiple languages:

### Building Documentation

```bash
# Build documentation
./build-docs.sh

# Serve documentation locally
cd docs && mdbook serve
# Open http://localhost:3000 in your browser
```

### Documentation Languages
- **English**
- **简体中文** (Simplified Chinese)
- **繁體中文** (Traditional Chinese)
- **Français** (French)
- **Deutsch** (German)
- **Русский** (Russian)

## Development Roadmap

- **Stage 1: Foundation Types** ✅
- **Stage 2: Topological Kernel** ✅
- **Stage 3: Modeling Algorithms** ✅
- **Stage 4: Data Exchange** ✅
- **Stage 5: Internationalization** ✅
- **Stage 6: Language Bindings** ✅
- **Stage 7: Mesh Generation** (in progress)
- **Stage 8: Visualization** (in progress)
- **Stage 9: Application Framework** (in progress)
- **Stage 10: Optimization** (planning)
- **Stage 11: Testing and Validation** (ongoing)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/mikewolfli/BrepRs.git
   cd BrepRs
   ```

2. Run tests:
   ```bash
   cargo test
   ```

3. Build the library:
   ```bash
   cargo build
   ```

4. Build language bindings:
   ```bash
   # Python
   cd bindings/python
   maturin develop --release
   
   # Node.js
   cd bindings/nodejs
   npm install && npm run build
   ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with Rust - a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety
- PyO3 for Python bindings
- NAPI-RS for Node.js bindings
- ext-php-rs for PHP bindings
- JNI for Java bindings
- mdbook for documentation

## Contact

- Project Link: [https://github.com/mikewolfli/BrepRs](https://github.com/mikewolfli/BrepRs)
- Author: Mike Wolfli
