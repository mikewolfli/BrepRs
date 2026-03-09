# BrepRs Coding Standards

## 1. Overview

This document defines the coding standards for the BrepRs project, which provides a Rust implementation of CAD kernel functionality with OpenCASCADE API compatibility.

## 2. Naming Conventions

### 2.1 Rust Native API (Primary API)

The primary Rust API follows standard Rust naming conventions:

- **Types (structs, enums, traits)**: `UpperCamelCase`
  - Examples: `TopoDsShape`, `BrepBuilder`, `ShapeType`, `NCollectionList`
  
- **Functions and methods**: `snake_case`
  - Examples: `new()`, `add_vertex()`, `compute_bounding_box()`
  
- **Variables**: `snake_case`
  - Examples: `shape_type`, `vertex_count`, `is_closed`
  
- **Constants**: `SCREAMING_SNAKE_CASE`
  - Examples: `STANDARD_REAL_EPSILON`, `MAX_ITERATIONS`
  
- **Modules**: `snake_case`
  - Examples: `topology`, `geometry`, `modeling`

### 2.2 OpenCASCADE Compatibility API

To maintain compatibility with OpenCASCADE's C++ API, we provide a separate compatibility layer that preserves the original naming:

- **Types**: Preserve original OpenCASCADE names with underscores
  - Examples: `TopoDS_Shape`, `BRep_Builder`, `gp_Pnt`, `BRepAlgoAPI_Fuse`
  
- **Functions**: Preserve original OpenCASCADE naming when applicable
  - Examples: `MakeVertex()`, `Build()`, `Perform()`

## 3. API Architecture

### 3.1 Dual API Design

The project provides two APIs:

1. **Rust Native API** (`src/`): Standard Rust naming, idiomatic Rust code
2. **OpenCASCADE Compatibility API** (`src/compat/`): OpenCASCADE naming for easy migration

### 3.2 File Organization

```
src/
├── topology/
│   ├── topods_shape.rs      # Rust native: TopoDsShape
│   └── compat.rs             # OCC compat: TopoDS_Shape
├── modeling/
│   ├── brep_builder.rs       # Rust native: BrepBuilder
│   └── compat.rs             # OCC compat: BRep_Builder
└── compat/                   # Dedicated compatibility module
    ├── mod.rs
    ├── topology.rs
    ├── modeling.rs
    └── geometry.rs
```

### 3.3 Implementation Pattern

The compatibility layer wraps the native Rust API:

```rust
// Native Rust API (topology/topods_shape.rs)
pub struct TopoDsShape {
    shape_type: ShapeType,
    // ...
}

impl TopoDsShape {
    pub fn new(shape_type: ShapeType) -> Self {
        // implementation
    }
}

// OpenCASCADE Compatibility API (compat/topology.rs)
pub use crate::topology::topods_shape::TopoDsShape as TopoDS_Shape;

pub struct TopoDS_Shape(pub(crate) TopoDsShape);

impl TopoDS_Shape {
    pub fn new(shape_type: ShapeType) -> Self {
        Self(TopoDsShape::new(shape_type))
    }
}
```

## 4. Documentation Requirements

### 4.1 Native API Documentation

- Use standard Rust doc comments (`///`)
- Include examples in documentation
- Document panics, errors, and safety considerations

### 4.2 Compatibility API Documentation

- Reference the original OpenCASCADE documentation
- Note any behavioral differences
- Provide migration examples

## 5. Compatibility Guidelines

### 5.1 Type Mapping

| OpenCASCADE Type | Rust Native Type | Compatibility Type |
|------------------|------------------|-------------------|
| TopoDS_Shape     | TopoDsShape      | compat::TopoDS_Shape |
| TopoDS_Vertex    | TopoDsVertex     | compat::TopoDS_Vertex |
| TopoDS_Edge      | TopoDsEdge       | compat::TopoDS_Edge |
| TopoDS_Wire      | TopoDsWire       | compat::TopoDS_Wire |
| TopoDS_Face      | TopoDsFace       | compat::TopoDS_Face |
| TopoDS_Shell     | TopoDsShell      | compat::TopoDS_Shell |
| TopoDS_Solid     | TopoDsSolid      | compat::TopoDS_Solid |
| TopoDS_Compound  | TopoDsCompound   | compat::TopoDS_Compound |
| TopoDS_CompSolid | TopoDsCompSolid  | compat::TopoDS_CompSolid |
| BRep_Builder     | BrepBuilder      | compat::BRep_Builder |
| gp_Pnt           | Point            | compat::gp_Pnt |
| gp_Vec           | Vector           | compat::gp_Vec |
| gp_Dir           | Direction        | compat::gp_Dir |

### 5.2 Method Mapping

When possible, maintain similar method signatures while adapting to Rust conventions:

```rust
// OpenCASCADE C++
TopoDS_Vertex BRepBuilderAPI_MakeVertex(const gp_Pnt& P);

// Rust Native
impl BrepBuilder {
    pub fn make_vertex(&self, point: &Point) -> TopoDsVertex;
}

// Compatibility
impl BRep_Builder {
    pub fn MakeVertex(&self, point: &gp_Pnt) -> TopoDS_Vertex;
}
```

## 6. Migration Path

### 6.1 For New Projects

Use the Rust Native API for idiomatic Rust code:

```rust
use breprs::topology::TopoDsShape;
use breprs::geometry::Point;

let shape = TopoDsShape::new(ShapeType::Vertex);
```

### 6.2 For OpenCASCADE Migration

Use the Compatibility API for easier migration:

```rust
use breprs::compat::{TopoDS_Shape, gp_Pnt, BRep_Builder};

let shape = TopoDS_Shape::new(ShapeType::Vertex);
```

## 7. Testing Requirements

### 7.1 Native API Tests

Located in `tests/native/` or inline with `#[cfg(test)]`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shape_creation() {
        let shape = TopoDsShape::new(ShapeType::Vertex);
        assert!(shape.is_vertex());
    }
}
```

### 7.2 Compatibility API Tests

Located in `tests/compat/`:

```rust
#[test]
fn test_occ_shape_creation() {
    use breprs::compat::TopoDS_Shape;
    
    let shape = TopoDS_Shape::new(ShapeType::Vertex);
    assert!(shape.IsVertex());
}
```

## 8. Version Compatibility

### 8.1 Semantic Versioning

- Major version changes may affect both APIs
- Minor version additions apply to both APIs
- Patch fixes apply to both APIs

### 8.2 Deprecation Policy

When deprecating functionality:
1. Mark as deprecated in both APIs
2. Provide migration documentation
3. Maintain for at least one major version

## 9. Code Quality

### 9.1 Linting

All code must pass:
```bash
cargo check
cargo clippy -- -D warnings
cargo fmt --check
```

### 9.2 Documentation

All public APIs must have documentation:
```bash
cargo doc --no-deps
```

## 10. References

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [OpenCASCADE Documentation](https://dev.opencascade.org/doc/overview/html/)
- [Rust Naming Conventions](https://rust-lang.github.io/api-guidelines/naming.html)
