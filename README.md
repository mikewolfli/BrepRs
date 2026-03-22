# BrepRs

[![Build Status](https://img.shields.io/github/actions/workflow/status/mikewolfli/BrepRs/rust.yml?branch=main)](https://github.com/mikewolfli/BrepRs/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/breprs.svg)](https://crates.io/crates/breprs)

BrepRs is a Rust implementation of a Boundary Representation (BRep) solid modeling library. It provides a comprehensive set of tools for creating, manipulating, and analyzing 3D geometric models.

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

### Technical Highlights
- **Rust-Based**: Leverages Rust's safety, performance, and concurrency features
- **Modular Architecture**: Clean, modular design with well-defined interfaces
- **Comprehensive Testing**: Extensive unit tests for all modules

## Project Structure

```
src/
├── foundation/          # Foundation types and utilities
│   ├── types.rs         # Basic numeric types
│   ├── string.rs        # String handling
│   ├── handle.rs        # Smart pointers for topology
│   ├── exception.rs     # Exception handling
│   └── memory.rs        # Memory management
├── topology/            # Topological kernel
│   ├── topods_shape.rs  # Base shape class
│   ├── topods_vertex.rs # Vertex implementation
│   ├── topods_edge.rs   # Edge implementation
│   ├── topods_wire.rs   # Wire implementation
│   ├── topods_face.rs   # Face implementation
│   ├── topods_shell.rs  # Shell implementation
│   ├── topods_solid.rs  # Solid implementation
│   ├── topods_compound.rs # Compound implementation
│   ├── topods_compsolid.rs # CompSolid implementation
│   ├── shape_enum.rs    # Shape type enumeration
│   └── explorer.rs      # Shape traversal tools
├── geometry/            # Geometric primitives
│   ├── point.rs         # Point implementation
│   ├── vector.rs        # Vector implementation
│   ├── direction.rs     # Direction implementation
│   ├── line.rs          # Line implementation
│   ├── circle.rs        # Circle implementation
│   ├── ellipse.rs       # Ellipse implementation
│   ├── plane.rs         # Plane implementation
│   ├── surface.rs       # Surface base class
│   ├── curve.rs         # Curve base class
│   └── transform.rs     # Transformation utilities
├── modeling/            # Modeling algorithms
│   ├── primitive_creation.rs # Primitive shape creation
│   ├── boolean_operations.rs # Boolean operations
│   ├── fillet_chamfer.rs # Fillet and chamfer operations
│   └── offset_operations.rs # Offset operations
├── data_exchange/       # File format support
│   ├── stl.rs           # STL file format
│   ├── step.rs          # STEP file format
│   ├── iges.rs          # IGES file format
│   └── mod.rs           # Data exchange module
├── mesh/                # Mesh generation (in progress)
├── visualization/       # Visualization (in progress)
├── application/         # Application framework (in progress)
└── lib.rs               # Library entry point
```

## Getting Started

### Prerequisites
- Rust 1.70+ (stable)
- Cargo (Rust package manager)

### Installation

Add BrepRs to your `Cargo.toml`:

```toml
[dependencies]
breprs = "0.1.0"
```

### Basic Usage

```rust
use breprs::topology::*;
use breprs::modeling::primitive_creation::PrimitiveCreator;
use breprs::data_exchange::stl::StlWriter;

fn main() {
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
    
    println!("Primitives created and saved to STL files!");
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

## Development Roadmap

- **Stage 1: Foundation Types** ✅
- **Stage 2: Topological Kernel** ✅
- **Stage 3: Modeling Algorithms** ✅
- **Stage 4: Data Exchange** ✅
- **Stage 5: Mesh Generation** (in progress)
- **Stage 6: Visualization** (in progress)
- **Stage 7: Application Framework** (in progress)
- **Stage 7.5: Optimization** (planning)
- **Stage 8: Testing and Validation** (ongoing)

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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with Rust - a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety

## Contact

- Project Link: [https://github.com/mikewolfli/BrepRs](https://github.com/mikewolfli/BrepRs)
- Author: Mike Wolfli
