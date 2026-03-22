# Core Concepts

This section introduces the core concepts of BrepRs, including geometry, topology, and modeling operations.

## Geometry

Geometry in BrepRs represents the mathematical representation of points, vectors, curves, and surfaces.

### Points

A point is a zero-dimensional geometric entity defined by its coordinates in 3D space.

### Vectors

A vector represents a direction and magnitude in 3D space.

### Curves

Curves represent one-dimensional geometric entities, such as lines, circles, ellipses, and splines.

### Surfaces

Surfaces represent two-dimensional geometric entities, such as planes, cylinders, spheres, cones, and toruses.

## Topology

Topology in BrepRs represents the connectivity of geometric entities.

### Vertices

Vertices are zero-dimensional topological entities corresponding to points in geometry.

### Edges

Edges are one-dimensional topological entities corresponding to curves in geometry.

### Faces

Faces are two-dimensional topological entities corresponding to surfaces in geometry.

### Shells

Shells are collections of faces that form a closed boundary.

### Solids

Solids are three-dimensional topological entities bounded by shells.

## Boolean Operations

Boolean operations allow you to combine shapes using operations like fuse, cut, common, and section.

### Fuse

Combines two shapes into one by merging their volumes.

### Cut

Subtracts the volume of one shape from another.

### Common

Keeps only the volume common to both shapes.

### Section

Creates a shape representing the intersection of two shapes.

## Surface Modeling

Surface modeling allows you to create and manipulate surfaces.

### Extrusion

Creates a surface by extruding a curve along a vector.

### Revolution

Creates a surface by rotating a curve around an axis.

### Loft

Creates a surface by lofting between multiple curves.

### Sweep

Creates a surface by sweeping a curve along another curve.

## Mesh Generation

Mesh generation converts BRep shapes into mesh representations for rendering and simulation.

### Triangulation

Converts surfaces into triangles for rendering.

### Quad Meshing

Converts surfaces into quadrilaterals for simulation.

### Mesh Optimization

Improves mesh quality by optimizing vertex positions and element shapes.

## Next Steps

After understanding the core concepts, you may want to:

- **Explore the API Reference** - Learn how to use the BrepRs API
- **Check the Examples** - See code samples for common operations
- **Read the Advanced Topics** - Learn about advanced features
