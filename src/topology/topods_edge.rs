use crate::foundation::handle::Handle;
use crate::geometry::{Point, Vector};
use crate::topology::{
    topods_location::TopoDsLocation, topods_shape::TopoDsShape, topods_vertex::TopoDsVertex,
};

/// Trait for curves that can be associated with edges
///
/// Curves are reference-counted via Handle<T> to allow sharing between multiple edges.
pub trait Curve: std::fmt::Debug + Send + Sync {
    /// Get the point on the curve at a parameter value
    fn value(&self, parameter: f64) -> Point;

    /// Get the derivative (tangent) at a parameter value
    fn derivative(&self, parameter: f64) -> Vector;

    /// Get the parameter range of the curve
    fn parameter_range(&self) -> (f64, f64);
}

/// Represents an edge in topological structure
///
/// An edge is a curve bounded by two vertices. It can be
/// degenerate (both vertices are the same) or can be open
/// (infinite curve) or closed (loop).
///
/// # Curve Ownership and Lifetime
/// - The edge holds a `Handle<dyn Curve>` which is a thread-safe reference-counted pointer
/// - Multiple edges can share the same curve instance
/// - The curve will be automatically dropped when all handles to it are dropped
/// - Curves must implement `Send + Sync` to be used in a `Handle`
///
/// # Invariants
/// - An edge must have exactly two vertices (which may be the same for degenerate edges)
/// - If the edge has a curve, the vertices must lie on the curve (within tolerance)
/// - Tolerance must be non-negative
/// - Orientation must be either 1 (forward) or -1 (reversed)
///
/// # Usage Patterns
/// - Edges are typically created through BRepBuilder or primitive operations
/// - Use `Handle<TopoDsEdge>` for sharing edges across multiple wires
/// - Edges can be degenerate (both vertices equal) for representing points
/// - Edges without a curve represent straight line segments between vertices
#[derive(Debug)]
pub struct TopoDsEdge {
    shape: TopoDsShape,
    curve: Option<Handle<dyn Curve>>,
    vertices: [Handle<TopoDsVertex>; 2],
    tolerance: f64,
    orientation: i32,
}

impl TopoDsEdge {
    /// Create a new edge with two vertices
    pub fn new(vertex1: Handle<TopoDsVertex>, vertex2: Handle<TopoDsVertex>) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Edge),
            curve: None,
            vertices: [vertex1, vertex2],
            tolerance: 0.001,
            orientation: 1,
        }
    }

    /// Create a new edge with specified curve
    pub fn with_curve(
        vertex1: Handle<TopoDsVertex>,
        vertex2: Handle<TopoDsVertex>,
        curve: Handle<dyn Curve>,
    ) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Edge),
            curve: Some(curve),
            vertices: [vertex1, vertex2],
            tolerance: 0.001,
            orientation: 1,
        }
    }

    /// Create a new edge with tolerance
    pub fn with_tolerance(
        vertex1: Handle<TopoDsVertex>,
        vertex2: Handle<TopoDsVertex>,
        tolerance: f64,
    ) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Edge),
            curve: None,
            vertices: [vertex1, vertex2],
            tolerance,
            orientation: 1,
        }
    }

    /// Get the first vertex
    pub fn vertex1(&self) -> &Handle<TopoDsVertex> {
        &self.vertices[0]
    }

    /// Get the second vertex
    pub fn vertex2(&self) -> &Handle<TopoDsVertex> {
        &self.vertices[1]
    }

    /// Get both vertices as a slice
    pub fn vertices(&self) -> &[Handle<TopoDsVertex>] {
        &self.vertices
    }

    /// Get the curve of the edge
    pub fn curve(&self) -> Option<&Handle<dyn Curve>> {
        self.curve.as_ref()
    }

    /// Set the curve of the edge
    pub fn set_curve(&mut self, curve: Option<Handle<dyn Curve>>) {
        self.curve = curve;
    }

    /// Set the vertices of the edge
    pub fn set_vertices(&mut self, vertices: [Handle<TopoDsVertex>; 2]) {
        self.vertices = vertices;
    }

    /// Get the tolerance of the edge
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Set the tolerance of the edge
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }

    /// Get the orientation of the edge
    pub fn orientation(&self) -> i32 {
        self.orientation
    }

    /// Set the orientation of the edge
    pub fn set_orientation(&mut self, orientation: i32) {
        self.orientation = orientation;
    }

    /// Get the shape base
    pub fn shape(&self) -> &TopoDsShape {
        &self.shape
    }

    /// Get mutable reference to shape base
    pub fn shape_mut(&mut self) -> &mut TopoDsShape {
        &mut self.shape
    }

    /// Get the location of the edge
    pub fn location(&self) -> Option<&TopoDsLocation> {
        self.shape.location()
    }

    /// Set the location of the edge
    pub fn set_location(&mut self, location: TopoDsLocation) {
        self.shape.set_location(location);
    }

    /// Check if this edge is degenerate (both vertices are the same)
    pub fn is_degenerate(&self) -> bool {
        self.vertices[0] == self.vertices[1]
    }

    /// Check if this edge has a curve
    pub fn has_curve(&self) -> bool {
        self.curve.is_some()
    }

    /// Get the length of the edge
    pub fn length(&self) -> f64 {
        if self.is_degenerate() {
            return 0.0;
        }

        let v1 = self.vertices[0].point();
        let v2 = self.vertices[1].point();

        let dx = v2.x - v1.x;
        let dy = v2.y - v1.y;
        let dz = v2.z - v1.z;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Get the midpoint of the edge
    pub fn midpoint(&self) -> Point {
        if self.is_degenerate() {
            return *self.vertices[0].point();
        }

        let v1 = self.vertices[0].point();
        let v2 = self.vertices[1].point();

        Point::new(
            (v1.x + v2.x) / 2.0,
            (v1.y + v2.y) / 2.0,
            (v1.z + v2.z) / 2.0,
        )
    }

    /// Get the direction vector of the edge
    pub fn direction(&self) -> Vector {
        if self.is_degenerate() {
            return Vector::new(0.0, 0.0, 0.0);
        }

        let v1 = self.vertices[0].point();
        let v2 = self.vertices[1].point();

        Vector::new(v2.x - v1.x, v2.y - v1.y, v2.z - v1.z)
    }

    /// Get the unique identifier of the edge
    pub fn shape_id(&self) -> i32 {
        self.shape.shape_id()
    }

    /// Set the unique identifier of the edge
    pub fn set_shape_id(&mut self, id: i32) {
        self.shape.set_shape_id(id);
    }

    /// Check if this edge is mutable
    pub fn is_mutable(&self) -> bool {
        self.shape.is_mutable()
    }

    /// Set the mutability of the edge
    pub fn set_mutable(&mut self, mutable: bool) {
        self.shape.set_mutable(mutable);
    }

    /// Check if this edge contains a point
    pub fn contains(&self, point: &Point) -> bool {
        if self.is_degenerate() {
            return false;
        }

        let v1 = self.vertices[0].point();
        let v2 = self.vertices[1].point();

        let edge_vec = Vector::new(v2.x - v1.x, v2.y - v1.y, v2.z - v1.z);
        let point_vec = Vector::new(point.x - v1.x, point.y - v1.y, point.z - v1.z);

        let cross = edge_vec.cross(&point_vec);
        let dot = edge_vec.dot(&point_vec);

        if cross.magnitude() < self.tolerance {
            return dot >= 0.0 && dot <= edge_vec.magnitude();
        }

        false
    }

    /// Get the parameter value for a point on the edge
    pub fn parameter(&self, point: &Point) -> Option<f64> {
        if self.is_degenerate() {
            return Some(0.0);
        }

        let v1 = self.vertices[0].point();
        let v2 = self.vertices[1].point();

        let edge_vec = Vector::new(v2.x - v1.x, v2.y - v1.y, v2.z - v1.z);
        let point_vec = Vector::new(point.x - v1.x, point.y - v1.y, point.z - v1.z);

        let edge_len_sq = edge_vec.dot(&edge_vec);

        if edge_len_sq < self.tolerance * self.tolerance {
            return None;
        }

        Some(edge_vec.dot(&point_vec) / edge_len_sq)
    }
}

impl Default for TopoDsEdge {
    fn default() -> Self {
        let origin = std::sync::Arc::new(TopoDsVertex::new(Point::origin()));
        Self::new(Handle::new(origin.clone()), Handle::new(origin))
    }
}

impl Clone for TopoDsEdge {
    fn clone(&self) -> Self {
        Self {
            shape: self.shape.clone(),
            curve: self.curve.clone(),
            vertices: self.vertices.clone(),
            tolerance: self.tolerance,
            orientation: self.orientation,
        }
    }
}

impl PartialEq for TopoDsEdge {
    fn eq(&self, other: &Self) -> bool {
        self.shape_id() == other.shape_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_creation() {
        let v1 = Handle::new(std::sync::Arc::new(TopoDsVertex::new(Point::new(
            0.0, 0.0, 0.0,
        ))));
        let v2 = Handle::new(std::sync::Arc::new(TopoDsVertex::new(Point::new(
            1.0, 0.0, 0.0,
        ))));
        let edge = TopoDsEdge::new(v1.clone(), v2.clone());

        assert!(!edge.is_degenerate());
        assert_eq!(edge.vertex1(), &v1);
        assert_eq!(edge.vertex2(), &v2);
    }

    #[test]
    fn test_edge_degenerate() {
        let v = Handle::new(std::sync::Arc::new(TopoDsVertex::new(Point::new(
            1.0, 0.0, 0.0,
        ))));
        let edge = TopoDsEdge::new(v.clone(), v.clone());

        assert!(edge.is_degenerate());
        assert_eq!(edge.length(), 0.0);
    }

    #[test]
    fn test_edge_length() {
        let v1 = Handle::new(std::sync::Arc::new(TopoDsVertex::new(Point::new(
            0.0, 0.0, 0.0,
        ))));
        let v2 = Handle::new(std::sync::Arc::new(TopoDsVertex::new(Point::new(
            3.0, 0.0, 0.0,
        ))));
        let edge = TopoDsEdge::new(v1.clone(), v2.clone());

        let length = edge.length();
        assert!((length - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_edge_midpoint() {
        let v1 = Handle::new(std::sync::Arc::new(TopoDsVertex::new(Point::new(
            0.0, 0.0, 0.0,
        ))));
        let v2 = Handle::new(std::sync::Arc::new(TopoDsVertex::new(Point::new(
            2.0, 0.0, 0.0,
        ))));
        let edge = TopoDsEdge::new(v1.clone(), v2.clone());

        let midpoint = edge.midpoint();
        assert!((midpoint.x - 1.0).abs() < 0.001);
        assert!((midpoint.y - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_edge_direction() {
        let v1 = Handle::new(std::sync::Arc::new(TopoDsVertex::new(Point::new(
            0.0, 0.0, 0.0,
        ))));
        let v2 = Handle::new(std::sync::Arc::new(TopoDsVertex::new(Point::new(
            1.0, 1.0, 0.0,
        ))));
        let edge = TopoDsEdge::new(v1.clone(), v2.clone());

        let direction = edge.direction();
        assert!((direction.x - 1.0).abs() < 0.001);
        assert!((direction.y - 1.0).abs() < 0.001);
        assert_eq!(direction.z, 0.0);
    }

    #[test]
    fn test_edge_tolerance() {
        let v1 = Handle::new(std::sync::Arc::new(TopoDsVertex::new(Point::new(
            0.0, 0.0, 0.0,
        ))));
        let v2 = Handle::new(std::sync::Arc::new(TopoDsVertex::new(Point::new(
            1.0, 0.0, 0.0,
        ))));
        let edge = TopoDsEdge::with_tolerance(v1.clone(), v2.clone(), 0.01);

        assert_eq!(edge.tolerance(), 0.01);
    }

    #[test]
    fn test_edge_shape_id() {
        let v1 = Handle::new(std::sync::Arc::new(TopoDsVertex::new(Point::new(
            0.0, 0.0, 0.0,
        ))));
        let v2 = Handle::new(std::sync::Arc::new(TopoDsVertex::new(Point::new(
            1.0, 0.0, 0.0,
        ))));
        let mut edge = TopoDsEdge::new(v1.clone(), v2.clone());

        // shape_id is now auto-generated, so it should not be 0
        let initial_id = edge.shape_id();
        assert!(initial_id > 0);

        edge.set_shape_id(42);
        assert_eq!(edge.shape_id(), 42);
    }

    #[test]
    fn test_edge_clone() {
        let v1 = Handle::new(std::sync::Arc::new(TopoDsVertex::new(Point::new(
            0.0, 0.0, 0.0,
        ))));
        let v2 = Handle::new(std::sync::Arc::new(TopoDsVertex::new(Point::new(
            1.0, 0.0, 0.0,
        ))));
        let mut edge1 = TopoDsEdge::new(v1.clone(), v2.clone());
        edge1.set_shape_id(10);

        let edge2 = edge1.clone();
        assert_eq!(edge2.shape_id(), 10);
        assert_eq!(edge1, edge2);
    }
}
