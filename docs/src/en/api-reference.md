# API Reference

This section provides detailed documentation for the BrepRs API, including geometry, topology, modeling, and I/O operations.

## Geometry API

The geometry API provides classes and functions for working with geometric entities like points, vectors, curves, and surfaces.

### Points

- `Point` - Represents a point in 3D space
  - `new(x: f64, y: f64, z: f64)` - Creates a new point
  - `x() -> f64` - Gets the x-coordinate
  - `y() -> f64` - Gets the y-coordinate
  - `z() -> f64` - Gets the z-coordinate
  - `distance(other: &Point) -> f64` - Calculates the distance to another point

### Vectors

- `Vector` - Represents a vector in 3D space
  - `new(x: f64, y: f64, z: f64)` - Creates a new vector
  - `x() -> f64` - Gets the x-component
  - `y() -> f64` - Gets the y-component
  - `z() -> f64` - Gets the z-component
  - `length() -> f64` - Calculates the length of the vector
  - `normalize() -> Vector` - Normalizes the vector to unit length
  - `dot(other: &Vector) -> f64` - Calculates the dot product with another vector
  - `cross(other: &Vector) -> Vector` - Calculates the cross product with another vector

### Curves

- `Curve` - Represents a curve in 3D space
  - `line(start: &Point, end: &Point) -> Curve` - Creates a line segment
  - `circle(center: &Point, radius: f64, normal: &Vector) -> Curve` - Creates a circle
  - `ellipse(center: &Point, major_radius: f64, minor_radius: f64, normal: &Vector) -> Curve` - Creates an ellipse

### Surfaces

- `Surface` - Represents a surface in 3D space
  - `plane(origin: &Point, normal: &Vector) -> Surface` - Creates a plane
  - `cylinder(origin: &Point, radius: f64, height: f64, direction: &Vector) -> Surface` - Creates a cylinder
  - `sphere(center: &Point, radius: f64) -> Surface` - Creates a sphere
  - `cone(origin: &Point, radius: f64, height: f64, direction: &Vector) -> Surface` - Creates a cone
  - `torus(center: &Point, major_radius: f64, minor_radius: f64, normal: &Vector) -> Surface` - Creates a torus

## Topology API

The topology API provides classes and functions for working with topological entities like vertices, edges, faces, shells, and solids.

### Shapes

- `TopoDS_Shape` - Base class for all topological shapes
  - `shape_type() -> ShapeType` - Gets the type of the shape
  - `is_null() -> bool` - Checks if the shape is null
  - `copy() -> TopoDS_Shape` - Creates a copy of the shape
  - `clear() -> ()` - Clears the shape

### Vertices

- `TopoDS_Vertex` - Represents a vertex
  - `new() -> TopoDS_Vertex` - Creates a new vertex
  - `point() -> Point` - Gets the point associated with the vertex

### Edges

- `TopoDS_Edge` - Represents an edge
  - `new() -> TopoDS_Edge` - Creates a new edge
  - `curve() -> Curve` - Gets the curve associated with the edge
  - `first_vertex() -> TopoDS_Vertex` - Gets the first vertex of the edge
  - `last_vertex() -> TopoDS_Vertex` - Gets the last vertex of the edge

### Faces

- `TopoDS_Face` - Represents a face
  - `new() -> TopoDS_Face` - Creates a new face
  - `surface() -> Surface` - Gets the surface associated with the face
  - `outer_wire() -> TopoDS_Wire` - Gets the outer wire of the face

### Shells

- `TopoDS_Shell` - Represents a shell
  - `new() -> TopoDS_Shell` - Creates a new shell
  - `add(face: &TopoDS_Face) -> ()` - Adds a face to the shell

### Solids

- `TopoDS_Solid` - Represents a solid
  - `new() -> TopoDS_Solid` - Creates a new solid
  - `add(shell: &TopoDS_Shell) -> ()` - Adds a shell to the solid

## Modeling API

The modeling API provides functions for creating and manipulating shapes.

### Primitives

- `make_box(width: f64, height: f64, depth: f64, origin: Option<Point>) -> TopoDS_Shape` - Creates a box
- `make_sphere(radius: f64, center: Option<Point>) -> TopoDS_Shape` - Creates a sphere
- `make_cylinder(radius: f64, height: f64, origin: Option<Point>) -> TopoDS_Shape` - Creates a cylinder
- `make_cone(radius: f64, height: f64, origin: Option<Point>) -> TopoDS_Shape` - Creates a cone
- `make_torus(major_radius: f64, minor_radius: f64, center: Option<Point>) -> TopoDS_Shape` - Creates a torus

### Boolean Operations

- `fuse(shape1: &TopoDS_Shape, shape2: &TopoDS_Shape) -> Result<TopoDS_Shape, Error>` - Fuses two shapes
- `cut(shape1: &TopoDS_Shape, shape2: &TopoDS_Shape) -> Result<TopoDS_Shape, Error>` - Cuts shape2 from shape1
- `common(shape1: &TopoDS_Shape, shape2: &TopoDS_Shape) -> Result<TopoDS_Shape, Error>` - Finds the common volume of two shapes
- `section(shape1: &TopoDS_Shape, shape2: &TopoDS_Shape) -> Result<TopoDS_Shape, Error>` - Creates a section of two shapes

### Surface Operations

- `extrude(shape: &TopoDS_Shape, direction: &Vector, distance: f64) -> Result<TopoDS_Shape, Error>` - Extrudes a shape
- `revolve(shape: &TopoDS_Shape, axis: &Vector, angle: f64) -> Result<TopoDS_Shape, Error>` - Revolves a shape
- `loft(curves: &[Curve]) -> Result<TopoDS_Shape, Error>` - Creates a loft between curves
- `sweep(profile: &TopoDS_Shape, path: &Curve) -> Result<TopoDS_Shape, Error>` - Sweeps a profile along a path

## IO API

The IO API provides functions for reading and writing shapes to files.

### Reading

- `read_step(filename: &str) -> Result<TopoDS_Shape, Error>` - Reads a STEP file
- `read_iges(filename: &str) -> Result<TopoDS_Shape, Error>` - Reads an IGES file
- `read_stl(filename: &str) -> Result<TopoDS_Shape, Error>` - Reads an STL file
- `read_obj(filename: &str) -> Result<TopoDS_Shape, Error>` - Reads an OBJ file
- `read_gltf(filename: &str) -> Result<TopoDS_Shape, Error>` - Reads a glTF file

### Writing

- `write_step(shape: &TopoDS_Shape, filename: &str) -> Result<(), Error>` - Writes a STEP file
- `write_iges(shape: &TopoDS_Shape, filename: &str) -> Result<(), Error>` - Writes an IGES file
- `write_stl(shape: &TopoDS_Shape, filename: &str) -> Result<(), Error>` - Writes an STL file
- `write_obj(shape: &TopoDS_Shape, filename: &str) -> Result<(), Error>` - Writes an OBJ file
- `write_gltf(shape: &TopoDS_Shape, filename: &str) -> Result<(), Error>` - Writes a glTF file

## I18n API

The I18n API provides functions for internationalization and localization.

- `I18n::init() -> ()` - Initializes the internationalization system
- `I18n::set_language(language: Language) -> ()` - Sets the current language
- `I18n::current_language() -> Language` - Gets the current language
- `I18n::available_languages() -> Vec<Language>` - Gets the available languages
- `I18n::tr(key: MessageKey) -> String` - Translates a message key to the current language

## Next Steps

After exploring the API reference, you may want to:

- **Check the Examples** - See code samples for common operations
- **Read the Advanced Topics** - Learn about advanced features
- **Build Your Application** - Start developing with BrepRs
