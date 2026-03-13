//! Chainable Builder API
//!
//! This module provides a method chaining API for building and modifying shapes.
//! The API follows the builder pattern with fluent interface design.
//!
//! # Examples
//!
//! ```
//! use breprs::api::chainable::ChainableBuilder;
//!
//! let solid = ChainableBuilder::new()
//!     .box_shape(10.0, 10.0, 10.0)
//!     .fillet(1.0)
//!     .translate(Vector::new(5.0, 0.0, 0.0))
//!     .build();
//! ```

use crate::foundation::handle::Handle;
use crate::geometry::{Axis, Direction, Matrix, Point, Transform, Vector};
use crate::modeling::boolean_operations::BooleanOperations;
use crate::modeling::fillet_chamfer::FilletChamfer;
use crate::modeling::offset_operations::OffsetOperations;
use crate::modeling::primitives;
use crate::topology::{
    topods_compound::TopoDsCompound, topods_edge::TopoDsEdge, topods_face::TopoDsFace,
    topods_shape::TopoDsShape, topods_shell::TopoDsShell, topods_solid::TopoDsSolid,
    topods_wire::TopoDsWire,
};

/// Chainable builder for constructing and modifying shapes
#[derive(Debug, Clone)]
pub struct ChainableBuilder {
    shape: Option<Handle<TopoDsShape>>,
    operations: Vec<String>,
}

impl ChainableBuilder {
    /// Create a new empty chainable builder
    pub fn new() -> Self {
        Self {
            shape: None,
            operations: Vec::new(),
        }
    }

    /// Create a builder from an existing shape
    pub fn from_shape(shape: Handle<TopoDsShape>) -> Self {
        Self {
            shape: Some(shape),
            operations: Vec::new(),
        }
    }

    // =========================================================================
    // Primitive Creation
    // =========================================================================

    /// Create a box shape
    pub fn box_shape(mut self, width: f64, height: f64, depth: f64) -> Self {
        let solid = primitives::make_box(width, height, depth, None);
        self.shape = Some(Handle::new(std::sync::Arc::new(solid.shape().clone())));
        self.operations
            .push(format!("box({}, {}, {})", width, height, depth));
        self
    }

    /// Create a box shape at a specific position
    pub fn box_at(mut self, width: f64, height: f64, depth: f64, position: Point) -> Self {
        let solid = primitives::make_box(width, height, depth, Some(position));
        self.shape = Some(Handle::new(std::sync::Arc::new(solid.shape().clone())));
        self.operations.push(format!(
            "box_at({}, {}, {}, {:?})",
            width, height, depth, position
        ));
        self
    }

    /// Create a sphere shape
    pub fn sphere(mut self, radius: f64) -> Self {
        let solid = primitives::make_sphere(radius, None);
        self.shape = Some(Handle::new(std::sync::Arc::new(solid.shape().clone())));
        self.operations.push(format!("sphere({})", radius));
        self
    }

    /// Create a sphere at a specific position
    pub fn sphere_at(mut self, radius: f64, center: Point) -> Self {
        let solid = primitives::make_sphere(radius, Some(center));
        self.shape = Some(Handle::new(std::sync::Arc::new(solid.shape().clone())));
        self.operations
            .push(format!("sphere_at({}, {:?})", radius, center));
        self
    }

    /// Create a cylinder shape
    pub fn cylinder(mut self, radius: f64, height: f64) -> Self {
        let solid = primitives::make_cylinder(radius, height, None, None);
        self.shape = Some(Handle::new(std::sync::Arc::new(solid.shape().clone())));
        self.operations
            .push(format!("cylinder({}, {})", radius, height));
        self
    }

    /// Create a cylinder at a specific position
    pub fn cylinder_at(mut self, radius: f64, height: f64, position: Point) -> Self {
        let solid = primitives::make_cylinder(radius, height, Some(position), None);
        self.shape = Some(Handle::new(std::sync::Arc::new(solid.shape().clone())));
        self.operations.push(format!(
            "cylinder_at({}, {}, {:?})",
            radius, height, position
        ));
        self
    }

    /// Create a cone shape
    pub fn cone(mut self, radius1: f64, radius2: f64, height: f64) -> Self {
        let solid = primitives::make_cone(radius1, radius2, height, None, None);
        self.shape = Some(Handle::new(std::sync::Arc::new(solid.shape().clone())));
        self.operations
            .push(format!("cone({}, {}, {})", radius1, radius2, height));
        self
    }

    /// Create a torus shape
    pub fn torus(mut self, major_radius: f64, minor_radius: f64) -> Self {
        let solid = primitives::make_torus(major_radius, minor_radius, None);
        self.shape = Some(Handle::new(std::sync::Arc::new(solid.shape().clone())));
        self.operations
            .push(format!("torus({}, {})", major_radius, minor_radius));
        self
    }

    /// Create a prism shape
    pub fn prism(mut self, profile: Handle<TopoDsWire>, height: f64) -> Self {
        // Convert Handle<TopoDsWire> to &TopoDsWire
        let wire_ref = profile.as_ref().expect("Handle<TopoDsWire> is empty");
        let vector = crate::geometry::Vector::new(0.0, 0.0, height);
        let solid = primitives::make_prism(wire_ref, &vector);
        self.shape = Some(Handle::new(std::sync::Arc::new(solid.shape().clone())));
        self.operations.push(format!("prism({})", height));
        self
    }

    /// Create a revolution shape
    pub fn revolution(mut self, profile: Handle<TopoDsWire>, angle: f64) -> Self {
        // Convert Handle<TopoDsWire> to &TopoDsWire
        let wire_ref = profile.as_ref().expect("Handle<TopoDsWire> is empty");
        let axis = crate::geometry::Axis::z_axis();
        let solid = primitives::make_revolution(wire_ref, &axis, angle);
        self.shape = Some(Handle::new(std::sync::Arc::new(solid.shape().clone())));
        self.operations.push(format!("revolution({})", angle));
        self
    }

    // =========================================================================
    // Boolean Operations
    // =========================================================================

    /// Union (fuse) with another shape
    pub fn fuse(mut self, other: &ChainableBuilder) -> Self {
        if let (Some(ref shape1), Some(ref shape2)) = (&self.shape, &other.shape) {
            let boolean_ops = BooleanOperations::new();
            let result = boolean_ops.fuse(shape1, shape2);
            self.shape = Some(Handle::new(std::sync::Arc::new(result.shape().clone())));
            self.operations.push("fuse".to_string());
        }
        self
    }

    /// Subtract (cut) another shape from this one
    pub fn cut(mut self, other: &ChainableBuilder) -> Self {
        if let (Some(ref shape1), Some(ref shape2)) = (&self.shape, &other.shape) {
            let boolean_ops = BooleanOperations::new();
            let result = boolean_ops.cut(shape1, shape2);
            self.shape = Some(Handle::new(std::sync::Arc::new(result.shape().clone())));
            self.operations.push("cut".to_string());
        }
        self
    }

    /// Intersection (common) with another shape
    pub fn intersect(mut self, other: &ChainableBuilder) -> Self {
        if let (Some(ref shape1), Some(ref shape2)) = (&self.shape, &other.shape) {
            let boolean_ops = BooleanOperations::new();
            let result = boolean_ops.common(shape1, shape2);
            self.shape = Some(Handle::new(std::sync::Arc::new(result.shape().clone())));
            self.operations.push("intersect".to_string());
        }
        self
    }

    /// Section with a plane
    pub fn section(mut self, point: Point, normal: Direction) -> Self {
        if let Some(ref shape) = self.shape {
            let boolean_ops = BooleanOperations::new();
            use crate::geometry::Plane;
            let plane = Plane::from_point_normal(point, normal);
            let result = boolean_ops.section_with_plane(shape, &plane);
            self.shape = Some(Handle::new(std::sync::Arc::new(result.shape().clone())));
            self.operations.push("section".to_string());
        }
        self
    }

    // =========================================================================
    // Fillet and Chamfer Operations
    // =========================================================================

    /// Apply fillet to all edges
    pub fn fillet(mut self, radius: f64) -> Self {
        if let Some(ref shape) = self.shape {
            if let Some(shape_ref) = shape.get() {
                if shape_ref.is_solid() {
                    let arc = shape.get().unwrap().clone();
                    if let Ok(solid) = arc.downcast::<TopoDsSolid>() {
                        let solid_ref = solid.as_ref();
                        let edges = solid_ref.edges();
                        let fillet_ops = FilletChamfer::with_radius(radius);
                        let result = fillet_ops.fillet_edges(solid_ref, &edges, radius);
                        self.shape = Some(Handle::new(std::sync::Arc::new(result.shape().clone())));
                        self.operations.push(format!("fillet({})", radius));
                    }
                }
            }
        }
        self
    }

    /// Apply chamfer to all edges
    pub fn chamfer(mut self, distance: f64) -> Self {
        if let Some(ref shape) = self.shape {
            if let Some(shape_ref) = shape.get() {
                if shape_ref.is_solid() {
                    let arc = shape.get().unwrap().clone();
                    if let Ok(solid) = arc.downcast::<TopoDsSolid>() {
                        let solid_ref = solid.as_ref();
                        let faces = solid_ref.faces();
                        let chamfer_ops = FilletChamfer::with_chamfer_distance(distance);
                        let result = chamfer_ops.chamfer_faces(solid_ref, &faces, distance);
                        self.shape = Some(Handle::new(std::sync::Arc::new(result.shape().clone())));
                        self.operations.push(format!("chamfer({})", distance));
                    }
                }
            }
        }
        self
    }

    // =========================================================================
    // Offset Operations
    // =========================================================================

    /// Offset the shape by a distance
    pub fn offset(mut self, distance: f64) -> Self {
        if let Some(ref shape) = self.shape {
            if let Some(shape_ref) = shape.get() {
                let arc = shape.get().unwrap().clone();
                if shape_ref.is_solid() {
                    if let Ok(solid) = arc.downcast::<TopoDsSolid>() {
                        let solid_ref = solid.as_ref();
                        let offset_ops = OffsetOperations::with_offset_distance(distance);
                        if let Some(shell) = solid_ref.shells().first() {
                            if let Some(shell_ref) = shell.get() {
                                let result = offset_ops.offset_shell(shell_ref, distance);
                                let mut new_solid = TopoDsSolid::new();
                                new_solid.add_shell(Handle::new(std::sync::Arc::new(result)));
                                self.shape = Some(Handle::new(std::sync::Arc::new(
                                    new_solid.shape().clone(),
                                )));
                                self.operations.push(format!("offset({})", distance));
                            }
                        }
                    }
                } else if shape_ref.is_face() {
                    if let Ok(face) = arc.downcast::<TopoDsFace>() {
                        let face_ref = face.as_ref();
                        let offset_ops = OffsetOperations::with_offset_distance(distance);
                        let result = offset_ops.offset_face(face_ref, distance);
                        self.shape = Some(Handle::new(std::sync::Arc::new(result.shape().clone())));
                        self.operations.push(format!("offset({})", distance));
                    }
                }
            }
        }
        self
    }

    /// Thicken the shape
    pub fn thicken(mut self, thickness: f64) -> Self {
        if let Some(ref shape) = self.shape {
            if let Some(shape_ref) = shape.get() {
                let arc = shape.get().unwrap().clone();
                if shape_ref.is_solid() {
                    if let Ok(solid) = arc.downcast::<TopoDsSolid>() {
                        let solid_ref = solid.as_ref();
                        let offset_ops = OffsetOperations::with_offset_distance(thickness);
                        if let Some(shell) = solid_ref.shells().first() {
                            if let Some(shell_ref) = shell.get() {
                                let result = offset_ops.make_thick_solid(shell_ref, thickness, 1.0);
                                self.shape =
                                    Some(Handle::new(std::sync::Arc::new(result.shape().clone())));
                                self.operations.push(format!("thicken({})", thickness));
                            }
                        }
                    }
                } else if shape_ref.is_face() {
                    if let Ok(face) = arc.downcast::<TopoDsFace>() {
                        let face_ref = face.as_ref();
                        let offset_ops = OffsetOperations::with_offset_distance(thickness);
                        let result = offset_ops.make_thick_from_face(face_ref, thickness, 1.0);
                        self.shape = Some(Handle::new(std::sync::Arc::new(result.shape().clone())));
                        self.operations.push(format!("thicken({})", thickness));
                    }
                }
            }
        }
        self
    }

    // =========================================================================
    // Transform Operations
    // =========================================================================

    /// Translate the shape
    pub fn translate(mut self, vector: Vector) -> Self {
        if let Some(ref shape) = self.shape {
            if let Some(shape_ref) = shape.get() {
                let mut new_shape = shape_ref.clone();
                let transform = Transform::from_translation(&vector);
                new_shape.transform(&transform);
                self.shape = Some(Handle::new(std::sync::Arc::new(new_shape)));
                self.operations.push(format!("translate({:?})", vector));
            }
        }
        self
    }

    /// Rotate the shape
    pub fn rotate(mut self, axis: Axis, angle: f64) -> Self {
        if let Some(ref shape) = self.shape {
            if let Some(shape_ref) = shape.get() {
                let mut new_shape = shape_ref.clone();
                let transform = Transform::from_rotation(&axis, angle);
                new_shape.transform(&transform);
                self.shape = Some(Handle::new(std::sync::Arc::new(new_shape)));
                self.operations
                    .push(format!("rotate({:?}, {})", axis, angle));
            }
        }
        self
    }

    /// Scale the shape uniformly
    pub fn scale(mut self, factor: f64) -> Self {
        if let Some(ref shape) = self.shape {
            if let Some(shape_ref) = shape.get() {
                let mut new_shape = shape_ref.clone();
                let transform = Transform::from_scale(factor);
                new_shape.transform(&transform);
                self.shape = Some(Handle::new(std::sync::Arc::new(new_shape)));
                self.operations.push(format!("scale({})", factor));
            }
        }
        self
    }

    /// Scale the shape non-uniformly
    pub fn scale_xyz(mut self, sx: f64, sy: f64, sz: f64) -> Self {
        if let Some(ref shape) = self.shape {
            if let Some(shape_ref) = shape.get() {
                let mut new_shape = shape_ref.clone();
                let scaling_matrix =
                    Matrix::from_column_major([sx, 0.0, 0.0], [0.0, sy, 0.0], [0.0, 0.0, sz]);
                let transform = Transform::from_matrix(&scaling_matrix);
                new_shape.transform(&transform);
                self.shape = Some(Handle::new(std::sync::Arc::new(new_shape)));
                self.operations
                    .push(format!("scale_xyz({}, {}, {})", sx, sy, sz));
            }
        }
        self
    }

    /// Mirror the shape
    pub fn mirror(mut self, point: Point, normal: Direction) -> Self {
        if let Some(ref shape) = self.shape {
            if let Some(shape_ref) = shape.get() {
                let mut new_shape = shape_ref.clone();
                let axis = Axis::new(point, normal);
                let transform = Transform::from_axis_mirror(&axis);
                new_shape.transform(&transform);
                self.shape = Some(Handle::new(std::sync::Arc::new(new_shape)));
                self.operations
                    .push(format!("mirror({:?}, {:?})", point, normal));
            }
        }
        self
    }

    // =========================================================================
    // Build and Output
    // =========================================================================

    /// Build and return the final shape
    pub fn build(self) -> Option<Handle<TopoDsShape>> {
        self.shape
    }

    /// Build and return as a solid (if applicable)
    pub fn build_solid(self) -> Option<Handle<TopoDsSolid>> {
        self.shape.and_then(|s| {
            if s.as_ref().map_or(false, |shape| shape.is_solid()) {
                let arc = s.get().unwrap().clone();
                // Try downcasting Arc<TopoDsShape> to Arc<TopoDsSolid>
                arc.downcast::<TopoDsSolid>().ok().map(Handle::new)
            } else {
                None
            }
        })
    }

    /// Build and return as a compound
    pub fn build_compound(self) -> Option<Handle<TopoDsCompound>> {
        self.shape.and_then(|s| {
            if s.as_ref().map_or(false, |shape| shape.is_compound()) {
                let arc = s.get().unwrap().clone();
                arc.downcast::<TopoDsCompound>().ok().map(Handle::new)
            } else {
                None
            }
        })
    }

    /// Get the operation history
    pub fn operation_history(&self) -> &[String] {
        &self.operations
    }

    /// Check if the builder has a shape
    pub fn has_shape(&self) -> bool {
        self.shape.is_some()
    }
}

impl Default for ChainableBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Specialized Builders
// =========================================================================

/// Builder specifically for creating solids
pub struct SolidBuilder {
    builder: ChainableBuilder,
}

impl SolidBuilder {
    pub fn new() -> Self {
        Self {
            builder: ChainableBuilder::new(),
        }
    }

    pub fn box_shape(self, width: f64, height: f64, depth: f64) -> Self {
        Self {
            builder: self.builder.box_shape(width, height, depth),
        }
    }

    pub fn sphere(self, radius: f64) -> Self {
        Self {
            builder: self.builder.sphere(radius),
        }
    }

    pub fn cylinder(self, radius: f64, height: f64) -> Self {
        Self {
            builder: self.builder.cylinder(radius, height),
        }
    }

    pub fn fillet(self, radius: f64) -> Self {
        Self {
            builder: self.builder.fillet(radius),
        }
    }

    pub fn chamfer(self, distance: f64) -> Self {
        Self {
            builder: self.builder.chamfer(distance),
        }
    }

    pub fn translate(self, vector: Vector) -> Self {
        Self {
            builder: self.builder.translate(vector),
        }
    }

    pub fn rotate(self, axis: Axis, angle: f64) -> Self {
        Self {
            builder: self.builder.rotate(axis, angle),
        }
    }

    pub fn build(self) -> Option<Handle<TopoDsSolid>> {
        self.builder.build_solid()
    }
}

impl Default for SolidBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating complex assemblies
pub struct AssemblyBuilder {
    shapes: Vec<ChainableBuilder>,
}

impl AssemblyBuilder {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    /// Add a shape to the assembly
    pub fn add(mut self, shape: ChainableBuilder) -> Self {
        self.shapes.push(shape);
        self
    }

    /// Combine all shapes into a compound
    pub fn build(self) -> ChainableBuilder {
        let mut compound_builder = ChainableBuilder::new();

        for shape_builder in self.shapes {
            if let Some(shape) = shape_builder.build() {
                // In a real implementation, we would combine these into a compound
            }
        }

        compound_builder
    }
}

impl Default for AssemblyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chainable_builder_box() {
        let builder = ChainableBuilder::new().box_shape(10.0, 10.0, 10.0);

        assert!(builder.has_shape());
        assert_eq!(builder.operation_history().len(), 1);
    }

    #[test]
    fn test_chainable_builder_sphere() {
        let builder = ChainableBuilder::new().sphere(5.0);

        assert!(builder.has_shape());
        assert_eq!(builder.operation_history().len(), 1);
    }

    #[test]
    fn test_chainable_builder_chaining() {
        let builder = ChainableBuilder::new()
            .box_shape(10.0, 10.0, 10.0)
            .fillet(1.0)
            .translate(Vector::new(5.0, 0.0, 0.0));

        assert!(builder.has_shape());
        assert_eq!(builder.operation_history().len(), 3);
    }

    #[test]
    fn test_solid_builder() {
        let solid = SolidBuilder::new()
            .box_shape(10.0, 10.0, 10.0)
            .fillet(1.0)
            .build();

        assert!(solid.is_some());
    }
}
