use crate::foundation::handle::Handle;
use crate::geometry::Point;
use crate::topology::{
    topods_location::TopoDsLocation, topods_shape::TopoDsShape, topods_wire::TopoDsWire,
};
use std::sync::Arc;

/// Represents a face in topological structure
///
/// A face is a bounded portion of a surface, bounded by one or more wires.
/// The first wire is the outer boundary, and subsequent wires are holes.
#[derive(Debug)]
pub struct TopoDsFace {
    shape: TopoDsShape,
    surface: Option<Handle<dyn Surface>>,
    wires: Vec<Handle<TopoDsWire>>,
    tolerance: f64,
    orientation: i32,
}

impl TopoDsFace {
    /// Create a new empty face
    pub fn new() -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Face),
            surface: None,
            wires: Vec::new(),
            tolerance: 0.001,
            orientation: 1,
        }
    }

    /// Create a new face with specified surface
    pub fn with_surface(surface: Handle<dyn Surface>) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Face),
            surface: Some(surface),
            wires: Vec::new(),
            tolerance: 0.001,
            orientation: 1,
        }
    }

    /// Create a new face with specified wires
    pub fn with_wires(wires: Vec<Handle<TopoDsWire>>) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Face),
            surface: None,
            wires,
            tolerance: 0.001,
            orientation: 1,
        }
    }

    /// Create a new face with surface and wires
    pub fn with_surface_and_wires(
        surface: Handle<dyn Surface>,
        wires: Vec<Handle<TopoDsWire>>,
    ) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Face),
            surface: Some(surface),
            wires,
            tolerance: 0.001,
            orientation: 1,
        }
    }

    /// Create a new face with outer wire
    pub fn with_outer_wire(wire: TopoDsWire) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Face),
            surface: None,
            wires: vec![Handle::new(Arc::new(wire))],
            tolerance: 0.001,
            orientation: 1,
        }
    }

    /// Create a new face with specified tolerance
    pub fn with_tolerance(tolerance: f64) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Face),
            surface: None,
            wires: Vec::new(),
            tolerance,
            orientation: 1,
        }
    }

    /// Add a wire to the face
    pub fn add_wire(&mut self, wire: Handle<TopoDsWire>) {
        self.wires.push(wire);
    }

    /// Get the wires of the face
    pub fn wires(&self) -> &[Handle<TopoDsWire>] {
        &self.wires
    }

    /// Get the number of wires in the face
    pub fn num_wires(&self) -> usize {
        self.wires.len()
    }

    /// Get the outer boundary wire (first wire)
    pub fn outer_wire(&self) -> Option<&Handle<TopoDsWire>> {
        self.wires.first()
    }

    /// Set the outer boundary wire (first wire)
    pub fn set_outer_wire(&mut self, wire: Handle<TopoDsWire>) {
        if self.wires.is_empty() {
            self.wires.push(wire);
        } else {
            self.wires[0] = wire;
        }
    }

    /// Get the hole wires (all wires except the first)
    pub fn hole_wires(&self) -> &[Handle<TopoDsWire>] {
        if self.wires.len() <= 1 {
            return &[];
        }
        &self.wires[1..]
    }

    /// Get the surface of the face
    pub fn surface(&self) -> Option<&Handle<dyn Surface>> {
        self.surface.as_ref()
    }

    /// Set the surface of the face
    pub fn set_surface(&mut self, surface: Handle<dyn Surface>) {
        self.surface = Some(surface);
    }

    /// Get the tolerance of the face
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Set the tolerance of the face
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }

    /// Get the orientation of the face
    pub fn orientation(&self) -> i32 {
        self.orientation
    }

    /// Set the orientation of the face
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

    /// Get the location of the face
    pub fn location(&self) -> Option<&TopoDsLocation> {
        self.shape.location()
    }

    /// Set the location of the face
    pub fn set_location(&mut self, location: TopoDsLocation) {
        self.shape.set_location(location);
    }

    /// Check if the face is empty (no wires)
    pub fn is_empty(&self) -> bool {
        self.wires.is_empty()
    }

    /// Check if the face has holes
    pub fn has_holes(&self) -> bool {
        self.wires.len() > 1
    }

    /// Check if the face has a surface
    pub fn has_surface(&self) -> bool {
        self.surface.is_some()
    }

    /// Clear all wires from the face
    pub fn clear(&mut self) {
        self.wires.clear();
    }

    /// Get the total area of the face
    pub fn area(&self) -> f64 {
        if let Some(outer_wire) = self.outer_wire() {
            self.wire_area(outer_wire)
        } else {
            0.0
        }
    }

    /// Calculate the area bounded by a wire
    fn wire_area(&self, wire: &Handle<TopoDsWire>) -> f64 {
        let vertices = wire.vertices();
        if vertices.len() < 3 {
            return 0.0;
        }

        let mut area = 0.0;
        let n = vertices.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let vi = vertices[i].point();
            let vj = vertices[j].point();
            area += vi.x * vj.y - vj.x * vi.y;
        }

        area.abs() / 2.0
    }

    /// Get the centroid of the face
    pub fn centroid(&self) -> Option<Point> {
        if let Some(outer_wire) = self.outer_wire() {
            self.wire_centroid(outer_wire)
        } else {
            None
        }
    }

    /// Calculate the centroid of a wire
    fn wire_centroid(&self, wire: &Handle<TopoDsWire>) -> Option<Point> {
        let vertices = wire.vertices();
        if vertices.is_empty() {
            return None;
        }

        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_z = 0.0;

        for vertex in vertices {
            let point = vertex.point();
            sum_x += point.x;
            sum_y += point.y;
            sum_z += point.z;
        }

        Some(Point::new(
            sum_x / vertices.len() as f64,
            sum_y / vertices.len() as f64,
            sum_z / vertices.len() as f64,
        ))
    }

    /// Get the unique identifier of the face
    pub fn shape_id(&self) -> i32 {
        self.shape.shape_id()
    }

    /// Set the unique identifier of the face
    pub fn set_shape_id(&mut self, id: i32) {
        self.shape.set_shape_id(id);
    }

    /// Check if this face is mutable
    pub fn is_mutable(&self) -> bool {
        self.shape.is_mutable()
    }

    /// Set the mutability of the face
    pub fn set_mutable(&mut self, mutable: bool) {
        self.shape.set_mutable(mutable);
    }

    /// Check if the face contains a specific wire
    pub fn contains_wire(&self, wire: &Handle<TopoDsWire>) -> bool {
        self.wires.contains(wire)
    }

    /// Get the bounding box of the face
    pub fn bounding_box(&self) -> Option<(Point, Point)> {
        if self.wires.is_empty() {
            return None;
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut min_z = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        let mut max_z = f64::MIN;

        for wire in &self.wires {
            if let Some((min, max)) = wire.bounding_box() {
                min_x = min_x.min(min.x);
                min_y = min_y.min(min.y);
                min_z = min_z.min(min.z);
                max_x = max_x.max(max.x);
                max_y = max_y.max(max.y);
                max_z = max_z.max(max.z);
            }
        }

        Some((
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        ))
    }

    /// Check if a point is inside the face
    pub fn contains_point(&self, point: &Point) -> bool {
        if let Some(outer_wire) = self.outer_wire() {
            self.point_in_wire(point, outer_wire)
        } else {
            false
        }
    }

    /// Check if a point is inside a wire (using ray casting)
    fn point_in_wire(&self, point: &Point, wire: &Handle<TopoDsWire>) -> bool {
        let vertices = wire.vertices();
        if vertices.len() < 3 {
            return false;
        }

        let mut inside = false;
        let n = vertices.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let vi = vertices[i].point();
            let vj = vertices[j].point();

            if ((vi.y > point.y) != (vj.y > point.y))
                && (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x)
            {
                inside = !inside;
            }
        }

        inside
    }

    /// Reverse the orientation of the face
    pub fn reverse(&mut self) {
        self.orientation *= -1;
    }
}

impl Default for TopoDsFace {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for TopoDsFace {
    fn clone(&self) -> Self {
        Self {
            shape: self.shape.clone(),
            surface: self.surface.clone(),
            wires: self.wires.clone(),
            tolerance: self.tolerance,
            orientation: self.orientation,
        }
    }
}

impl PartialEq for TopoDsFace {
    fn eq(&self, other: &Self) -> bool {
        self.shape_id() == other.shape_id()
    }
}

/// Trait for surfaces that can be associated with faces
pub trait Surface: std::fmt::Debug {
    /// Get the point on the surface at (u, v) parameters
    fn value(&self, u: f64, v: f64) -> Point;

    /// Get the normal at (u, v) parameters
    fn normal(&self, u: f64, v: f64) -> crate::geometry::Vector;

    /// Get the parameter range of the surface
    fn parameter_range(&self) -> ((f64, f64), (f64, f64));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_creation() {
        let face = TopoDsFace::new();
        assert!(face.is_empty());
        assert_eq!(face.num_wires(), 0);
    }

    #[test]
    fn test_face_add_wire() {
        let mut face = TopoDsFace::new();
        let wire = Handle::new(std::sync::Arc::new(TopoDsWire::new()));

        face.add_wire(wire);
        assert_eq!(face.num_wires(), 1);
        assert!(!face.has_holes());
    }

    #[test]
    fn test_face_has_holes() {
        let wire1 = Handle::new(std::sync::Arc::new(TopoDsWire::new()));
        let wire2 = Handle::new(std::sync::Arc::new(TopoDsWire::new()));
        let face = TopoDsFace::with_wires(vec![wire1, wire2]);

        assert!(face.has_holes());
    }

    #[test]
    fn test_face_outer_wire() {
        let wire1 = Handle::new(std::sync::Arc::new(TopoDsWire::new()));
        let wire2 = Handle::new(std::sync::Arc::new(TopoDsWire::new()));
        let face = TopoDsFace::with_wires(vec![wire1.clone(), wire2]);

        assert_eq!(face.outer_wire().unwrap().shape_id(), wire1.shape_id());
    }

    #[test]
    fn test_face_hole_wires() {
        let wire1 = Handle::new(std::sync::Arc::new(TopoDsWire::new()));
        let wire2 = Handle::new(std::sync::Arc::new(TopoDsWire::new()));
        let wire3 = Handle::new(std::sync::Arc::new(TopoDsWire::new()));
        let face = TopoDsFace::with_wires(vec![wire1, wire2.clone(), wire3.clone()]);

        let holes = face.hole_wires();
        assert_eq!(holes.len(), 2);
        assert_eq!(holes[0].shape_id(), wire2.shape_id());
        assert_eq!(holes[1].shape_id(), wire3.shape_id());
    }

    #[test]
    fn test_face_clear() {
        let wire = Handle::new(std::sync::Arc::new(TopoDsWire::new()));
        let mut face = TopoDsFace::with_wires(vec![wire]);
        assert!(!face.is_empty());

        face.clear();
        assert!(face.is_empty());
    }

    #[test]
    fn test_face_shape_id() {
        let mut face = TopoDsFace::new();
        // shape_id is now auto-generated, so it should not be 0
        let initial_id = face.shape_id();
        assert!(initial_id > 0);

        face.set_shape_id(42);
        assert_eq!(face.shape_id(), 42);
    }

    #[test]
    fn test_face_mutable() {
        let mut face = TopoDsFace::new();
        assert!(!face.is_mutable());

        face.set_mutable(true);
        assert!(face.is_mutable());
    }

    #[test]
    fn test_face_orientation() {
        let mut face = TopoDsFace::new();
        assert_eq!(face.orientation(), 1);

        face.set_orientation(-1);
        assert_eq!(face.orientation(), -1);
    }

    #[test]
    fn test_face_clone() {
        let mut face1 = TopoDsFace::new();
        face1.set_shape_id(10);

        let face2 = face1.clone();
        assert_eq!(face2.shape_id(), 10);
        assert_eq!(face1, face2);
    }
}
