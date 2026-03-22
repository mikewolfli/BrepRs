# Getting Started

This section will help you get started with BrepRs, including installation instructions and quick start guides.

## Installation

BrepRs can be installed through Cargo, Rust's package manager.

### Prerequisites

- Rust 1.70.0 or later
- Cargo (comes with Rust)
- Git (for cloning the repository)

### Installing from Crates.io

To use BrepRs in your Rust project, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
breprs = "0.6.0-alpha"
```

### Installing from Source

To install BrepRs from source, clone the repository and build it:

```bash
# Clone the repository
git clone https://github.com/mikewolfli/breprs.git
cd breprs

# Build the project
cargo build --release

# Run tests
cargo test
```

## Quick Start

This section provides a quick introduction to using BrepRs in your project.

### Basic Usage

Here's a simple example of how to use BrepRs:

```rust
use breprs::geometry::Point;
use breprs::topology::TopoDS_Shape;
use breprs::modeling::primitives::make_box;

fn main() {
    // Create a box
    let box_shape = make_box(1.0, 1.0, 1.0, None);
    
    // Get the shape
    let shape: TopoDS_Shape = box_shape.into_shape();
    
    // Print shape information
    println!("Shape type: {:?}", shape.shape_type());
    println!("Shape is null: {}", shape.is_null());
}
```

### Language Bindings

BrepRs provides bindings for multiple programming languages. See the [Language Bindings](language-bindings.md) section for more information on how to use BrepRs from other languages.

## Hello World

The [Hello World](getting-started/hello-world.md) section provides a step-by-step guide to creating your first BrepRs application.

## Next Steps

After getting started with BrepRs, you may want to explore:

- **Core Concepts** - Learn about BrepRs' geometry and topology concepts
- **API Reference** - Explore the complete API documentation
- **Examples** - See code samples for common use cases
- **Advanced Topics** - Learn about advanced features and techniques
